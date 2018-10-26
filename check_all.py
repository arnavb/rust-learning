#!/usr/bin/env python3

from pathlib import Path
from subprocess import call, Popen, PIPE
import sys


def execute_command(command, path):
    command = Popen(command, cwd=str(path), stdout=PIPE,
                    stderr=PIPE)
    stdout, stderr = command.communicate()
    exit_code = command.returncode

    return exit_code, stdout, stderr


def main():
    for directory in filter(lambda path: (path / 'Cargo.toml').exists(),
                            Path.cwd().glob('*/')):

        print("Checking project: {}".format(directory))

        exit_code, _, stderr = execute_command(['cargo', 'check'],
                                               directory)
        if exit_code != 0:
            print('Error! {} does not compile correctly '
                  '(Exit code: {})'.format(directory, exit_code))

            print('stderr:')
            print(stderr.decode('utf-8'))

            sys.exit(1)

        print("Successfully compiled project!")

        exit_code, *_ = execute_command(['cargo', 'clean'],
                                        directory)

        if exit_code != 0:
            print('Unable to clean built directory!')


if __name__ == '__main__':
    main()
