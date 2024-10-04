import os
import shutil
import json
import stat

from git import Repo

Repo.clone_from("https://gitlab.com/YuukiPS/GC-Proto.git", "GC-Proto")
Repo.clone_from(
    "https://github.com/juliuskreutz/reliquary-codegen.git", "reliquary-codegen"
)

os.mkdir("data")

shutil.copytree("GC-Proto/proto", "data/proto")

packetIds = {}

with open("GC-Proto/cmdid.json") as f:
    for packetId in json.load(f):
        packetIds[str(packetId["id"])] = packetId["name"]

with open("data/packetIds.json", "w") as f:
    json.dump(packetIds, f)

os.system("cd reliquary-codegen && cargo run -- ../. ../data")


def remove_readonly(func, path, excinfo):
    os.chmod(path, stat.S_IWRITE)
    func(path)


shutil.rmtree("GC-Proto", onexc=remove_readonly)
shutil.rmtree("reliquary-codegen", onexc=remove_readonly)
shutil.rmtree("data", onexc=remove_readonly)
