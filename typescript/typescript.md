## Security checklist 

### References

+ [Best practices for cybersecurity in TypeScript](https://crashtest-security.com/typescript-security/#best-practices-for-cybersecurity-in-typescript)

+ [Typescript Command Injection: Examples and Prevention](https://www.stackhawk.com/blog/typescript-command-injection-examples-and-prevention/)

+ [OWASP cheat sheet](https://cheatsheetseries.owasp.org/)

+ [OWASP Web Security Testing Guide](https://owasp.org/www-project-web-security-testing-guide/)



## Static analyzer for TS

- [codeQL](https://github.com/github/codeql)

    - Setup: https://docs.github.com/en/code-security/codeql-cli/using-the-codeql-cli/getting-started-with-the-codeql-cli
    - cd YOUR_PROJECT_FOLDER
    - codeql database create codeqldb --language=typescript
    - by adding "--overwrite" it will overwrite the existing database.

    - codeql database analyze --download --format=csv --output=./codeql-results.txt   codeqldb 'codeql/javascript-queries:codeql-suites/javascript-security-and-quality.qls'
    - Alternatively, codeql database analyze --download --format=sarif-latest --output=./codeql-results.sarif codeqldb 'codeql/javascript-queries:codeql-suites/javascript-security-and-quality.qls'

    - by adding "--download" it will allow the CLI to download any referenced CodeQL packages.



- [Semgrep](https://github.com/returntocorp/semgrep)
Semgrep is a fast, open-source, static analysis engine for finding bugs, detecting vulnerabilities in third-party dependencies, and enforcing code standards. 

`semgrep --config auto --output ./audit_logs/semgrep-result.emacs --emacs . --verbose`

- [snyk](https://docs.snyk.io/snyk-cli/getting-started-with-the-snyk-cli)
Snyk is a developer-first, cloud-native security tool to scan and monitor your software development projects for security vulnerabilities. 

`snyk code test`

- [typescript-eslint](https://github.com/typescript-eslint/typescript-eslint)
Monorepo for the tooling that enables ESLint and Prettier to support TypeScript. 

    1. install the required packages for ESLint, TypeScript, and our tooling:

    `npm install --save-dev eslint @eslint/js typescript typescript-eslint`

    2. create an eslint.config.js config file in the root of your project, and populate it with the following:

    ```
    // @ts-check

    import eslint from '@eslint/js';
    import tseslint from 'typescript-eslint';

    export default tseslint.config(
    eslint.configs.recommended,
    ...tseslint.configs.recommended,
    );
    ```

    3. Running ESLint
    ``npx eslint .``



- [ESLint](https://github.com/eslint/eslint)
ESLint is a tool for identifying and reporting on patterns found in ECMAScript/JavaScript code with language extensions (such as JSX, Flow, and TypeScript) on a case-by-case basis.
    - install and configuration: `$ npm init @eslint/config`
    - run ESLint on any file or directory: `npx eslint yourfile.js` or `yarn run eslint jourfile.js`

- [Mega Linter](https://megalinter.io/latest/)
MegaLinter is an Open-Source tool for CI/CD workflows that analyzes the consistency of your code, IAC, configuration, and scripts in your repository sources.


- [ts-dependency-graph](https://github.com/PSeitz/ts-dependency-graph)
Prints a dependency graph in dot format for your typescript or react project. Supported files are .ts and .tsx. Supports tsconfig path mappings since v1.4.3.
(e.g. ts_dependency_graph --start packages/keyring/src/index.ts --graph_folder | dot -T svg > dependencygraph.svg)



## Using Babel with TS

+ Use Babel and the whole ecosystem: using Babel instead of the TypeScript Checker makes transpiring much faster. It also enables you to use all the tools available in the Babel ecosystem. 
    + [Babel](https://babeljs.io/docs/)
    + [Using Babel with TypeScript](https://www.typescriptlang.org/docs/handbook/babel-with-typescript.html)


