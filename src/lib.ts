import assert from 'node:assert';
import { existsSync, readFileSync } from 'node:fs';
import { join } from 'node:path';
import type Decancer from './typings';

type Option<T> = T | undefined | null;

type Arch =
  | string
  | {
      name: string;
      musl: boolean;
    };

function isMusl(): boolean {
  // For Node 10;
  if (!process.report || typeof process.report.getReport !== 'function') {
    try {
      return readFileSync('/usr/bin/ldd', 'utf8').includes('musl');
    } catch {
      return true;
    }
  } else {
    // @ts-ignore
    const { glibcVersionRuntime } = process.report.getReport().header;
    return !glibcVersionRuntime;
  }
}

function handleModule(mod: any): Decancer {
  return Object.assign(mod.decancer, {
    contains: mod.contains
  });
}

function loadBinding(name: string): Decancer {
  if (existsSync(join(__dirname, '..', `decancer.${name}.node`))) {
    return handleModule(require(`../decancer.${name}.node`));
  }

  return handleModule(require(`@vierofernando/decancer-${name}`));
}

const platforms: Record<string, Record<string, Arch>> = {
  android: {
    arm64: 'android-arm64'
  },
  win32: {
    x64: 'win32-x64-msvc',
    ia32: 'win32-ia32-msvc',
    arm64: 'win32-arm64-msvc'
  },
  darwin: {
    x64: 'darwin-x64',
    arm64: 'darwin-arm64'
  },
  linux: {
    x64: {
      name: 'linux-x64',
      musl: true
    },
    arm64: {
      name: 'linux-arm64',
      musl: true
    },
    arm: 'linux-arm-gnueabihf'
  }
};

// for debugging purposes
if (existsSync(join(__dirname, '..', 'decancer.node'))) {
  module.exports = handleModule(require('../decancer.node'));
} else {
  try {
    const data: Option<Arch> = platforms[process.platform][process.arch];
    assert(data != null);

    if (typeof data === 'string') {
      module.exports = loadBinding(data);
    } else {
      if (data.musl && isMusl()) {
        module.exports = loadBinding(`${data.name}-musl`);
      } else {
        module.exports = loadBinding(`${data.name}-gnu`);
      }
    }
  } catch (err) {
    console.error(
      `error: cannot load module. OS: ${process.platform} Arch: ${process.arch} may not be supported.`
    );
    throw err;
  }
}
