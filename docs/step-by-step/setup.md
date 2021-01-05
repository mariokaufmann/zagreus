# Setup
In this section we will setup zagreus (if you haven't already) and create a zagreus template project. This tutorial assumes that you have a basic familiarity with the terminal / shell on your operating system.

## Setup zagreus
To install zagreus, you need to download the latest release from Github (TODO link). Unzip the archive to a location of your choosing. We recommend to add that location to the `PATH` environment variable on your system.

## Zagreus generator
The zagreus generator is a command-line utility to create, build and pack zagreus templates. If you have added the zagreus installation directory to the `PATH` you can start a terminal and use the generator without having to provide its absolue path:
```
zagreus-generator build
```
This will display an error message about a missing template project. This is expected, as we haven't created the project yet.

## Create zagreus project
With the zagreus generator we can generate a new template project. For this, we can use the `new` command. First, open a terminal and navigate to the folder where you want to create the new template. Then, issue the following command:
```
zagreus-generator new my-template
```
The zagreus generator will create a sub directory with the the name `my-template` and inside that directory set up a zagreus template project with the same name. The project consists of a few _.yaml_ configuration files and an asset subdirectory.
The file `zagreus-template.yaml` contains basic configuration options for the template such as its name for example. To find out what other configuration options are available please visit (TODO).

