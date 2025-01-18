import biomc_pp

root = "/home/benjamin/Documents/code/cpp/BioCMA-MCST/results/"

names = [
    "uptake",
    "uptake_22",
    "uptake_23",
    "uptake_24",
    "uptake_25",
    "uptake_26",
    "uptake_27",
    "uptake_28",
    "uptake_29",
]


pp = biomc_pp.ConcatPostProcess(names, root)
print(pp.time)
