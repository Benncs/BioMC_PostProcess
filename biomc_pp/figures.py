from abc import ABC, abstractmethod
import matplotlib.pyplot as plt
import plotly.graph_objects as go
from biomc_pp import get_time_unit
import numpy as np


def plt_hist(n, c, ax=None, **kwargs):
    if kwargs.get("density", False):
        c = c / (sum(c) * np.diff(n))  # Normalize bin heights
        kwargs.pop("density")

    if ax is None:
        fig, ax1 = plt.subplots()
        ax1.bar(n[:-1], c[:-1], width=np.diff(n), **kwargs)
        return fig, ax1
    else:
        ax.bar(n[:-1], c[:-1], width=np.diff(n), **kwargs)
        return ax


def plot_biomass_concentration(time, pp):
    x = np.sum(pp.get_biomass_concentration(), axis=1)
    title = "Biomass concentration according to time"
    xaxis_title = f"Time [{get_time_unit()}]"
    yaxis_title = "Biomass concentration [g]"
    fig = go.Figure()
    fig.add_trace(
        go.Scatter(
            x=time,
            y=x,
            mode="lines+markers",
            name="Biomass Concentration",
            line=dict(color="red"),  # Line color
            marker=dict(symbol="x", size=7, color="red"),  # Marker as small cross (x)
        )
    )

    #  template  ['ggplot2', 'seaborn', 'simple_white', 'plotly',
    # 'plotly_white', 'plotly_dark', 'presentation', 'xgridoff',
    # 'ygridoff', 'gridon', 'none']

    fig.update_layout(
        title=dict(text=title, x=0.5, xanchor="center", y=0.95),
        xaxis_title=xaxis_title,
        yaxis_title=yaxis_title,
        legend_title="Legend",
        font=dict(size=12, family="Arial"),  #
        showlegend=True,
        margin=dict(l=60, r=60, t=70, b=60),
        hovermode="closest",
        template="ggplot2",
        autosize=False,
        width=800,
        height=600,
    )
    fig.write_image("biomass_concentration_over_time.svg")
    return fig


def plot_local_biomass_concentration(time, pp, compartment):
    x = pp.get_biomass_concentration()[:, compartment]
    title = ("Biomass concentration according to time",)
    xaxis_title = f"Time [{get_time_unit()}]"
    yaxis_title = "Biomass concentration [g]"

    fig = go.Figure()
    fig.add_trace(
        go.Scatter(x=time, y=x, mode="lines+markers", name="Biomass Concentration")
    )
    fig.update_layout(
        title=title,
        xaxis_title=xaxis_title,
        yaxis_title=yaxis_title,
        template="simple_white",
    )

    return fig
