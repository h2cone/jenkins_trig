# jenkins_trig

A command-line tool designed to trigger Jenkins jobs and monitor their execution status.

## Build

To build the project, ensure you have Rust and Cargo installed. Then, follow these steps:

1. Clone the repository:

    ```shell
    git clone https://github.com/h2cone/jenkins_trig.git
    cd jenkins_trig
    ```

2. Build the project:

    ```shell
    cargo build -r
    ```

## Example

Here is an example of how to use the tool:

```shell
./jenkins_trig -v 'MyView' -j 'MyJob' -p 'key1=value1;key2=value2'
```

This command triggers the specified Jenkins job with the provided parameters and monitors its status.

## Configuration

This tool requires environment variables to specify the Jenkins base URL and the user credentials. You can use a `.env` file to set these configurations:

```text
JENKINS_URL=http://jenkins.example.com
JENKINS_USER=myuser
JENKINS_TOKEN=mytoken
```

Alternatively, you can set these environment variables directly in your `.bashrc` or `.zshrc` file:

```text
export JENKINS_URL=http://jenkins.example.com
export JENKINS_USER=myuser
export JEKNINS_TOKEN=mytoken
```

## Command-Line Arguments

* `-v` The view containing the job to be triggered.
* `-j` The name of the job to be triggered.
* `-p` The parameters to pass to the job, in the format 'key1=value1;key2=value2'.
* For more information try `--help`.