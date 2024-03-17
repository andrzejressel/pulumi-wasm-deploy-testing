// Copyright 2022, Pulumi Corporation.  All rights reserved.

package executors

import (
	"github.com/andrzejressel/pulumi-wasm/pulumi-language-wasm/fsys"
	"runtime"
)

type windows struct{}

var _ wasmExecutorFactory = &windows{}

func (s windows) NewWasmExecutor(opts WasmExecutorOptions) (*WasmExecutor, error) {
	if runtime.GOOS != "windows" {
		return nil, nil
	}
	probePaths := []string{opts.UseExecutor}
	if opts.UseExecutor == "" {
		probePaths = []string{".\\entrypoint.ps1"}
	}
	cmd, err := fsys.LookPath(opts.WD, probePaths...)
	if err != nil {
		return nil, err
	}
	return s.newWasmCliExecutor(cmd)
}

func (windows) newWasmCliExecutor(script string) (*WasmExecutor, error) {
	return &WasmExecutor{
		Name:        script,
		Cmd:         "powershell",
		BuildArgs:   []string{"-executionpolicy", "bypass", "-File", script, "build"},
		RunArgs:     []string{"-executionpolicy", "bypass", "-File", script, "run"},
		PluginArgs:  []string{"-File", script, "plugin"},
		VersionArgs: []string{"-File", script, "version"},
	}, nil
}
