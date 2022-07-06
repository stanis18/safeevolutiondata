## Experiment

In order to reproduce all experiments described in the paper you should have the [golang environment installed](https://towardsdev.com/golang-tutorial-2-installing-golang-on-linux-windows-and-mac-os-debf823eb699) and [docker](https://docs.docker.com/engine/install/).
Then you should build the solc-verify image in the Dockerfile-solcverify.src file and write the repository file path in searchDir variable on the main.go script. Finally you should execute the go run main.go command. The experiments will be processed and the results exported to a .csv spreadsheet.