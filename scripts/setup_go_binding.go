package main

import (
	"bytes"
	"errors"
	"fmt"
	"io"
	"os"
	"os/exec"
	"path/filepath"
	"runtime"
	"strings"
)

func isDir(dir string) bool {
	libDirStat, err := os.Stat(dir)

	return err == nil && libDirStat.IsDir()
}

func move(source, destination string) error {
	input, err := os.Open(source)
	inputClosed := false

	if err != nil {
		return err
	}

	defer func() {
		if !inputClosed {
			input.Close()
		}
	}()

	output, err := os.Create(destination)

	if err != nil {
		return err
	}

	defer output.Close()

	if _, err = io.Copy(output, input); err != nil {
		return err
	}

	inputClosed = true
	input.Close()

	return os.Remove(source)
}

func getSystemLibrariesPath() (string, error) {
	switch runtime.GOOS {
	case "windows":
		{
			gccPath, err := exec.LookPath("gcc")

			if err != nil {
				return "", errors.New("missing gcc compiler")
			}

			absGccPath, err := filepath.Abs(gccPath)

			if err != nil {
				return "", errors.New("missing gcc compiler")
			}

			mingwDir := filepath.Dir(absGccPath)

			for {
				libDir := filepath.Join(mingwDir, "lib")

				if isDir(libDir) {
					return libDir, nil
				}

				parentMingwDir := filepath.Dir(mingwDir)

				if parentMingwDir == mingwDir {
					return "", errors.New("unable to locate mingw lib directory")
				}

				mingwDir = parentMingwDir
			}
		}
	case "linux":
		{
			osRelease, err := os.ReadFile("/etc/os-release")
			systemLibrariesPath := "/usr/local/lib"

			if err == nil {
				osReleaseString := string(osRelease)

				if strings.Contains(osReleaseString, "ID=debian") || strings.Contains(osReleaseString, "ID_LIKE=debian") {
					systemLibrariesPath = "/usr/lib"
				}
			}

			return systemLibrariesPath, nil
		}
	case "darwin", "freebsd", "dragonfly", "netbsd", "openbsd", "illumos", "solaris":
		return "/usr/local/lib", nil
	case "aix", "hurd":
		return "/usr/lib", nil
	case "plan9":
		return "/sys/lib", nil
	default:
		return "", nil
	}
}

func build() error {
	_, filename, _, ok := runtime.Caller(0)

	if !ok {
		return errors.New("unable to retrieve current file's path")
	}

	bindingsPath := filepath.Join(filepath.Dir(filename), "..", "bindings")
	goBindingPath := filepath.Join(bindingsPath, "go")
	nativeBindingPath := filepath.Join(bindingsPath, "native")

	cargoBuildArgs := []string{"build", "--release", "--no-default-features", "--features", "options,separators,leetspeak,utf8"}
	nativeBinaryPath := []string{nativeBindingPath, "target", "", "release", "libdecancer.a"}

	if runtime.GOOS == "windows" {
		if err := exec.Command("gcc", "--version").Run(); err != nil {
			return errors.New("missing gcc compiler")
		}

		arch := runtime.GOARCH
		var rustArch string

		switch arch {
		case "amd64":
			rustArch = "x86_64"
		case "386":
			rustArch = "i686"
		case "arm64":
			rustArch = "aarch64"
		default:
			return errors.New("unsupported architecture: " + arch)
		}

		rustTarget := rustArch + "-pc-windows-gnu"
		nativeBinaryPath[2] = rustTarget

		addRustTargetCommand := exec.Command("rustup", "target", "add", rustTarget)
		addRustTargetCommand.Stdout = os.Stdout
		addRustTargetCommand.Stderr = os.Stderr

		cargoBuildArgs = append(cargoBuildArgs, "--target", rustTarget)

		if err := addRustTargetCommand.Run(); err != nil {
			return errors.New("unable to add rust target: " + err.Error())
		}
	}

	cargoTomlPath := filepath.Join(nativeBindingPath, "Cargo.toml")

	originalCargoToml, err := os.ReadFile(cargoTomlPath)

	if err != nil {
		return errors.New("unable to read \"" + cargoTomlPath + "\": " + err.Error())
	}

	modifiedCargoToml := bytes.Replace(originalCargoToml, []byte(`"cdylib"`), []byte(`"staticlib"`), 1)

	if err := os.WriteFile(cargoTomlPath, modifiedCargoToml, 0644); err != nil {
		return errors.New("unable to write \"" + cargoTomlPath + "\": " + err.Error())
	}

	defer func() {
		_ = os.WriteFile(cargoTomlPath, originalCargoToml, 0644)
	}()

	cargoBuildCommand := exec.Command("cargo", cargoBuildArgs...)
	cargoBuildCommand.Dir = nativeBindingPath
	cargoBuildCommand.Stdout = os.Stdout
	cargoBuildCommand.Stderr = os.Stderr

	if err := cargoBuildCommand.Run(); err != nil {
		return errors.New("unable to build native binding: " + err.Error())
	}

	nativeBinaryDestinationPath, err := getSystemLibrariesPath()

	if err != nil {
		return err
	} else if nativeBinaryDestinationPath == "" {
		nativeBinaryDestinationPath = goBindingPath
	} else {
		decancerGoPath := filepath.Join(goBindingPath, "decancer.go")
		originalDecancerGo, err := os.ReadFile(decancerGoPath)

		if err != nil {
			return errors.New("unable to read \"" + decancerGoPath + "\": " + err.Error())
		} else if err := os.WriteFile(decancerGoPath, bytes.Replace(originalDecancerGo, []byte(`-L${SRCDIR} `), []byte{}, 1), 0644); err != nil {
			return errors.New("unable to write \"" + decancerGoPath + "\": " + err.Error())
		}
	}

	if err := os.MkdirAll(nativeBinaryDestinationPath, 0755); err != nil {
		return errors.New("unable to create new directory \"" + nativeBinaryDestinationPath + "\": " + err.Error())
	} else if err := move(filepath.Join(nativeBinaryPath...), filepath.Join(nativeBinaryDestinationPath, "libdecancer.a")); err != nil {
		return errors.New("unable to move native binding binary to \"" + nativeBinaryDestinationPath + "\": " + err.Error())
	}

	fmt.Println("\"libdecancer.a\" has been added to \"" + nativeBinaryDestinationPath + "\"")

	return nil
}

func main() {
	if err := build(); err != nil {
		fmt.Fprintln(os.Stderr, "error:", err)
		os.Exit(1)
	}
}
