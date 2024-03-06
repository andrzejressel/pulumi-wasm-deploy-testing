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
		probePaths = []string{".\\entrypoint.bat"}
	}
	cmd, err := fsys.LookPath(opts.WD, probePaths...)
	if err != nil {
		return nil, err
	}
	return s.newWasmCliExecutor(cmd, opts.BootstrapLibJarPath)
}

func (windows) newWasmCliExecutor(script string, bootstrapLibJarPath string) (*WasmExecutor, error) {
	return &WasmExecutor{
		Name:        script,
		Cmd:         "cmd",
		BuildArgs:   []string{"/c", script, "build"},
		RunArgs:     []string{"/c", script, "run"},
		PluginArgs:  []string{"/c", script, "plugin"},
		VersionArgs: []string{"/c", script, "version"},
	}, nil
}
