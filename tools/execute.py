import json
import os.path
from subprocess import call

from paramiko import SSHClient

if os.path.isfile("buildserver.json"):
    file = open("buildserver.json", "r")
    buildserver = json.loads(file.read())
    file.close()
    ssh = SSHClient()
    ssh.load_system_host_keys()
    ssh.connect(buildserver["ip"], 22, buildserver["username"], buildserver["password"] if ~buildserver["askPassword"] else raw_input("Password? "))
    ssh_stdin, ssh_stdout, ssh_stderr = ssh.exec_command("bootimage build --target x86_64-rros.json")
    exit_status = ssh_stdout.channel.recv_exit_status()
    sftp = ssh.open_sftp()
    sftp.get("/workspace/rust/rros/target/x86_64-rros/debug/bootimage-rros.bin", "target/rros.bin")
    sftp.close()
    ssh.close()
    if exit_status == 0:
        call(["qemu-system-x86_64", "-drive", "format=raw,file=target/rros.bin"])
    else:
        print("Error", exit_status)