# Verify compliance

test_compliance.py is a python script to test functions of a library. The goal of this script is to test fastly the compliance by making a description of the algorithms (name, parameters, ...) to test against a reference (module cryptography for python). 

This script has been applied on a non compliant libolm that contains PBKDF2, and fastpbkdf2, an implementation found on github ( https://github.com/ctz/fastpbkdf2 ) that is compliant.

## Usage

The evaluator must find a way to make a library from the crypto code (you don't need to compile all the project, only the crypto code). 

Then for each algorithm, a json file must be created to fuzz the algorithm.

Note: \_\_attribute\_\_((visibility("default"))) permits to access functions from outside if it's not the case. See objdump -t lib.so to verify that the symbol can be accessed.

## JSON description
The JSON file contains the path and name of the lib targeted, and a list of algorithms to test. 

nb_tests is the number of tests per algorithm.

The arguments passed are in the order of the definition of the function in the C code. You can add constraints: give the value if it's constant, or give a minimum/maximum. The name of the arguments is given by the python script (see listRequiredParameters in the script).

constants is useful if you have constants that are not related to the arguments. For example, the PRF (pseudo random function) for PBKDF2 should be given so the good reference algorithm will be used.

returnCode can be removed or empty if there is no return code, but if there is one, the useful return code is when it's "OK". Others are not used.