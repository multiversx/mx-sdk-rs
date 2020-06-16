# Setting up the project

Creating a smart contract project from scratch by hand is possible but a fairly bad idea. The smart way is to start from a project template.

Instead of copying a project from somewhere, it is much more convenient to use erdpy templates.

To get a list of all the available templates, call:
```
erdpy templates
```
These templates are example projects created and maintained by the Elrond team.

The 'adder' is the simplest of these. We will use it as the basis for our project. Go to the directory where the project will be located and call:
```
erdpy new --template adder --directory ./target-dir easy-crowdfunder
```
Replace ./target-dir with the directory of your choice.

Now open the project in the IDE of your choice. For VSCode users this is
```
code ./target-dir/easy-crowdfunder
```

Let's look at the project layout:

* `debug` - Is a subproject that is required for debugging.
* `src` - Contains the actual smart contract sources.
* `target` - Is for the output binaries, don't worry about it.
* `test` - this is where unit and integration tests should be placed. More on testing later.
