// Copyright 2022, Pulumi Corporation.  All rights reserved.

package executors

import (
	"github.com/andrzejressel/pulumi-wasm/pulumi-language-wasm/fsys"
	"runtime"
)

type unix struct{}

var _ wasmExecutorFactory = &unix{}

func (s unix) NewWasmExecutor(opts WasmExecutorOptions) (*WasmExecutor, error) {
	if runtime.GOOS != "linux" && runtime.GOOS != "darwin" {
		return nil, nil
	}
	probePaths := []string{opts.UseExecutor}
	if opts.UseExecutor == "" {
		probePaths = []string{"./entrypoint.sh"}
	}
	cmd, err := fsys.LookPath(opts.WD, probePaths...)
	if err != nil {
		return nil, err
	}
	return s.newWasmCliExecutor(cmd)
}

func (unix) newWasmCliExecutor(script string) (*WasmExecutor, error) {
	return &WasmExecutor{
		Name:        script,
		Cmd:         "bash",
		BuildArgs:   []string{"-c", script, "build"},
		RunArgs:     []string{"-c", script, "run"},
		PluginArgs:  []string{"-c", script, "plugin"},
		VersionArgs: []string{"-c", script, "version"},
	}, nil
}
