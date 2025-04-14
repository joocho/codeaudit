import os
import ctypes
from cryptography.hazmat.primitives import hashes
from cryptography.hazmat.primitives.kdf.pbkdf2 import PBKDF2HMAC
import json
import sys
import random
import tqdm

# __attribute__((visibility("default"))) to export function

# definition of pbkdf2 in CRY.ME
# size_t compute_pbkdf2(uint8_t sha_type, uint32_t hlen,
#             const uint8_t *password, uint32_t password_length,
#             const uint8_t *salt, uint32_t salt_length,
#             uint32_t c,
#             uint8_t * DK, uint32_t dklen)


def cmp_array(a, b):
    """Compare two arrays, called to compare the results between reference lib
    and targeted lib"""
    if len(a) != len(b):
        return False
    for index in range(len(a)):
        if a[index] != b[index]:
            return False
    return True


def defineValue(content_algo):
    """Define values for all arguments,
    in others terms construct listParameters_ctypes to pass it to the
    targeted function"""
    dict_arg_type = {arg: dict_arg["type"] for (
        arg, dict_arg) in content_algo["arguments"].items()}
    listParameters_ctypes = []
    values = {}
    for (param_name, param_value) in dict_arg_type.items():
        if param_value[-1] == '*':
            # don't take pointers, they will be initialized outside of this
            # function (pass param_name for convenience)
            # set with 3 elts, just for convenience (it will be erased after)
            value = (None, param_name, param_value)
            listParameters_ctypes.append(value)
        else:
            # get constant/min/max just to lighten the code
            if "constant" in content_algo["arguments"][param_name].keys():
                isConstant = True
                value = content_algo["arguments"][param_name]["constant"]
            else:
                isConstant = False

            # find good type to set a value
            if param_value == "uint8_t":
                min_type = 0
                max_type = 255
                if isConstant:
                    # value is already set
                    pass
                else:
                    value = random.randint(
                        min_type if "min" not in
                        content_algo["arguments"][param_name] else
                        content_algo["arguments"][param_name]["min"],
                        max_type if "max" not in
                        content_algo["arguments"][param_name] else
                        content_algo["arguments"][param_name]["max"])
                listParameters_ctypes.append(ctypes.c_ubyte(value))
            elif param_value == "uint32_t":
                min_type = 0
                max_type = 1 << 32 - 1
                if isConstant:
                    # value is already set
                    pass
                else:
                    value = random.randint(
                        min_type if "min" not in
                        content_algo["arguments"][param_name] else
                        content_algo["arguments"][param_name]["min"],
                        max_type if "max" not in
                        content_algo["arguments"][param_name] else
                        content_algo["arguments"][param_name]["max"])
                listParameters_ctypes.append(ctypes.c_uint(value))
            else:
                raise ValueError(
                    f"{param_value} in {algorithm} is not supported")
        # save values
        values[param_name] = value

    # initialize pointers
    for index in range(len(listParameters_ctypes)):
        if isinstance(listParameters_ctypes[index], tuple) and listParameters_ctypes[index][0] is None:
            # we have an unitialized pointer
            param_name = listParameters_ctypes[index][1]
            param_type = listParameters_ctypes[index][2]
            if param_type == "uint8_t*":
                # get length that must have the parameter
                length = values["len_" + param_name]
                buffer = os.urandom(length)
                # adapt the structures
                listParameters_ctypes[index] = (
                    ctypes.c_ubyte * length)(*[x for x in buffer])
                values[param_name] = buffer

    return (listParameters_ctypes, values)


def verifyRequiredParameters(listRequiredParameters, content_algo):
    """Verify that required parameters are defined by user in json file"""
    dict_arg_type = {arg: dict_arg["type"] for (
        arg, dict_arg) in content_algo["arguments"].items()}
    for elt in listRequiredParameters:
        if elt not in list(content_algo["constants"].keys()) + list(content_algo["arguments"].keys()):
            raise ValueError(f"{elt} in {algorithm} is not given")
        if elt in content_algo["arguments"].keys():
            if dict_arg_type[elt][-1] == '*':
                # there should be a length given, parameter or constant (except for char*)
                if dict_arg_type[elt] != "char*" and "len_" + elt not in list(content_algo["arguments"].keys()) + list(content_algo["constants"].keys()):
                    raise ValueError(f"len_{elt} in {algorithm} is not given")
        else:
            # required parameter is in constant, ok
            pass
    return True


def testPBKDF2(content_algo, c_lib):
    # verify that all required parameters are passed, and length are passed (constant or argument)
    listRequiredParameters = ["password",
                              "salt", "iterations", "output", "PRF"]
    verifyRequiredParameters(listRequiredParameters, content_algo)

    # initialize integers (all except pointers) with random values
    (listParameters_ctypes, values) = defineValue(content_algo)

    # COMPUTATIONS

    # compute PBKDF2 with reference lib
    if content_algo["constants"]["PRF"] == "SHA1":
        kdf = PBKDF2HMAC(algorithm=hashes.SHA1(),
                         length=values["len_output"],
                         salt=values["salt"],
                         iterations=values["iterations"])
        derived = kdf.derive(values["password"])
    else:
        raise ValueError(
            f'{content_algo["constants"]["PRF"]} is not supported in PBKDF2')

    # compute PBKDF2 with lib targeted
    name_function = content_algo["name_function"]
    lib_function = getattr(c_lib, name_function)

    retcode = lib_function(*listParameters_ctypes)
    # FYI, ctypes supposes that functions return an int, but it's possible to change it with setrestype, see 
    # get good retcode
    if "returnCode" in content_algo and content_algo["returnCode"] != {}:
        goodRetCode = int([key for (key, value) in content_algo["returnCode"].items() if value == "OK" ][0])
        if retcode != goodRetCode:
            raise ValueError(f"PBKDF2 targeted is not OK, retcode={retcode}")

    # get output ctypes
    index_output_targeted = list(content_algo["arguments"].keys()).index("output")
    # TIME TO COMPARE
    if cmp_array(derived, listParameters_ctypes[index_output_targeted]):
        return True
    else:
        # get the input that breaks the targeted algorithm
        print("Compliance error, input given:")
        print(listParameters_ctypes)
        print(values)
        return False


if len(sys.argv) == 1:
    filename_conf = "description.json"
elif len(sys.argv) == 2:
    filename_conf = sys.argv[1]
else:
    print("Too much arguments")
    sys.exit(1)


# retrieve description of the library
with open(filename_conf, "r") as f:
    conf = json.load(f)

# Load the shared library into ctypes
libname = os.getcwd() + '/' + conf["path_lib"]
c_lib = ctypes.CDLL(libname)

for algorithm in conf["algorithms"].keys():
    content_algo = conf["algorithms"][algorithm]
    print(f"algorithm: {algorithm}")
    if algorithm == "PBKDF2":
        compliant = True
        for _ in tqdm.tqdm(range(content_algo["nb_tests"])):
            if not testPBKDF2(content_algo, c_lib):
                compliant = False
                break
        if compliant:
            print(f"{algorithm} compliant")
        else:
            print(f"{algorithm} NOT compliant")

    else:
        raise ValueError(f"The algorithm {algorithm} is not supported")
