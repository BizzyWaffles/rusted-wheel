# How To Use

First, make sure that you have Pulp and Bower installed.  Using Yarn:

```
yarn global add purescript pulp bower
```

then, get your dependencies installed from Bower

```
bower install
```

then, build the project, using

```
pulp build
```

for a basic build, or, to output to the expected location by the page:

```
pulp build --to output/main.js
```

or, for continuous builds:

```
pulp --watch build --to output/main.js
```

Then, if you open up `html/index.html` in your browser, your should have the page.

To run the tests, use `pulp test`.
