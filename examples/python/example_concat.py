import biomc_pp
from biomc_pp.figures import plot_biomass_concentration

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


pp = biomc_pp.PostProcess(names[1], root)

print(pp.get_property_names())

fig_x = plot_biomass_concentration(biomc_pp.check_time_unit(pp),pp)
fig_x.show()

# fig = make_subplots(rows=1, cols=2, 
#                         column_widths=[0.5, 0.5], 
#                         specs=[[{"type": "scatter"}, {"type": "scatter"}]])

#     # Add the first plot on the left
# fig.add_trace(fig_x.data[0], row=1, col=1)
    
#     # Add the second plot on the right
# fig.add_trace(fig_x.data[0], row=1, col=2)

# fig.show()