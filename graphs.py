import pandas as pd
import matplotlib.pyplot as plt
from pathlib import Path

US_TO_S = 1_000_000.0
DIR = Path("./csv-files")

# Add/rename these to match your actual filenames (one per Dilithium variant build)
FILES_METRIC1 = {
    "sr25519": "sr25519_measure_end_to_end.csv",
    "ecdsa": "ecdsa_measure_end_to_end.csv",
    "ml_dsa_44": "ml_dsa_44_measure_end_to_end.csv",
    "ml_dsa_65": "ml_dsa_65_measure_end_to_end.csv",
    "ml_dsa_87": "ml_dsa_87_measure_end_to_end.csv",
}

FILES_METRIC2 = {
    "sr25519": "sr25519_keygen.csv",
    "ecdsa": "ecdsa_keygen.csv",
    "ml_dsa_44": "ml_dsa_44_keygen.csv",
    "ml_dsa_65": "ml_dsa_65_keygen.csv",
    "ml_dsa_87": "ml_dsa_87_keygen.csv",
}

FILES_METRIC3 = {
    "sr25519": "sr25519_sign_transfer.csv",
    "ecdsa": "ecdsa_sign_transfer.csv",
    "ml_dsa_44": "ml_dsa_44_sign_transfer.csv",
    "ml_dsa_65": "ml_dsa_65_sign_transfer.csv",
    "ml_dsa_87": "ml_dsa_87_sign_transfer.csv",
}

# FILE_METRIC4 = "time_verification.csv"


# style
def set_plot_style():
    plt.rcParams.update({
        "figure.dpi": 120,
        "savefig.dpi": 160,
        "axes.grid": True,
        "grid.alpha": 0.25,
        "axes.spines.top": False,
        "axes.spines.right": False,
        "legend.frameon": False,
        "axes.titlepad": 10,
        "axes.labelpad": 8,
        "lines.linewidth": 1.8,
        "font.size": 10,
    })

def read_csv(path: str) -> pd.DataFrame:
    full_path = DIR / path
    if not full_path.exists():
        raise FileNotFoundError(f"Missing file: {full_path}")
    return pd.read_csv(full_path)

def load_algo_csvs(files: dict) -> dict:
    return {algo: read_csv(path) for algo, path in files.items()}

# helpers
def ensure_cols(df: pd.DataFrame, required: set, label: str):
    missing = sorted(required - set(df.columns))
    if missing:
        raise ValueError(f"{label}: missing columns {missing}")

def summarize(series: dict, value_col: str) -> pd.DataFrame:
    rows = []
    for alg, df in series.items():
        s = df[value_col].dropna()
        if len(s) == 0:
            continue
        rows.append({
            "algorithm": alg,
            "n": int(s.shape[0]),
            "median": float(s.median()),
            "p95": float(s.quantile(0.95)),
            "mean": float(s.mean()),
            "stdev": float(s.std(ddof=1)) if s.shape[0] > 1 else 0.0,
        })
    out = pd.DataFrame(rows).sort_values("mean")
    return out

def plot_timeseries(ax, series: dict, x_col: str, y_col: str, title: str, y_label: str, smooth_window: int = 0):
    for label, df in series.items():
        x = df[x_col]
        y = df[y_col]
        if smooth_window and smooth_window >= 3:
            y = y.rolling(smooth_window, center=True, min_periods=1).median()
        ax.plot(x, y, label=label)
    ax.set_title(title)
    ax.set_xlabel(x_col)
    ax.set_ylabel(y_label)
    ax.legend()

def plot_box(ax, series: dict, value_col: str, title: str, y_label: str):
    labels = []
    data = []
    for alg, df in series.items():
        vals = df[value_col].dropna().values
        if len(vals):
            labels.append(alg)
            data.append(vals)
    ax.boxplot(data, tick_labels=labels, showfliers=True)
    ax.set_title(title)
    ax.set_ylabel(y_label)

def plot_bar_mean(ax, series: dict, value_col: str, title: str, y_label: str, show_error: bool = False):
    # Mean bars; optional error bars (stdev)
    labels = []
    means = []
    errs = []
    for alg, df in series.items():
        vals = df[value_col].dropna()
        if len(vals) == 0:
            continue
        labels.append(alg)
        means.append(float(vals.mean()))
        errs.append(float(vals.std(ddof=1)) if len(vals) > 1 else 0.0)

    if not labels:
        ax.set_title(title)
        ax.set_ylabel(y_label)
        return

    x = list(range(len(labels)))
    if show_error:
        ax.bar(x, means, yerr=errs, capsize=3)
    else:
        ax.bar(x, means)

    ax.set_xticks(x, labels, rotation=20, ha="right")
    ax.set_title(title)
    ax.set_ylabel(y_label)


# metric 1 (ledger latency)
def plot_metric1_ledger(files: dict, smooth_window: int = 0):
    series = load_algo_csvs(files)
    required = {"iter", "included_us", "finalized_us", "all_included_us", "all_finalized_us"}
    for alg, df in series.items():
        ensure_cols(df, required, f"Metric1 {alg}")

    series_m1 = {}
    for alg, df in series.items():
        out = df.copy()
        out["included_s"] = out["included_us"] / US_TO_S
        out["finalized_s"] = out["finalized_us"] / US_TO_S
        out["all_included_s"] = out["all_included_us"] / US_TO_S
        out["all_finalized_s"] = out["all_finalized_us"] / US_TO_S
        out["overhead_included_ms"] = (out["all_included_us"] - out["included_us"]) / 1000.0
        out["overhead_finalized_ms"] = (out["all_finalized_us"] - out["finalized_us"]) / 1000.0
        series_m1[alg] = out

    fig, axes = plt.subplots(2, 3, figsize=(15, 7), constrained_layout=True)

    # --- OLD time-series (kept, but commented) ---
    # plot_timeseries(axes[0, 0], series_m1, "iter", "included_s",
    #                 "submit → included", "seconds", smooth_window)
    # plot_timeseries(axes[0, 1], series_m1, "iter", "finalized_s",
    #                 "submit → finalized", "seconds", smooth_window)
    # plot_timeseries(axes[0, 2], series_m1, "iter", "all_included_s",
    #                 "all (sign→) → included", "seconds", smooth_window)
    # plot_timeseries(axes[1, 0], series_m1, "iter", "all_finalized_s",
    #                 "all (sign→) → finalized", "seconds", smooth_window)
    # plot_timeseries(axes[1, 1], series_m1, "iter", "overhead_included_ms",
    #                 "overhead (all − submit) → included", "ms", smooth_window)
    # plot_timeseries(axes[1, 2], series_m1, "iter", "overhead_finalized_ms",
    #                 "overhead (all − submit) → finalized", "ms", smooth_window)

    # --- NEW: mean bar charts ---
    plot_bar_mean(axes[0, 0], series_m1, "included_s",
                  "submit → included (mean)", "seconds")
    plot_bar_mean(axes[0, 1], series_m1, "finalized_s",
                  "submit → finalized (mean)", "seconds")
    plot_bar_mean(axes[0, 2], series_m1, "all_included_s",
                  "all (sign→) → included (mean)", "seconds")

    plot_bar_mean(axes[1, 0], series_m1, "all_finalized_s",
                  "all (sign→) → finalized (mean)", "seconds")
    plot_bar_mean(axes[1, 1], series_m1, "overhead_included_ms",
                  "overhead (all − submit) → included (mean)", "ms")
    plot_bar_mean(axes[1, 2], series_m1, "overhead_finalized_ms",
                  "overhead (all − submit) → finalized (mean)", "ms")

    fig.suptitle("Metric 1 — Ledger latency overview (means)", y=1.02)

    # Distribution view: overheads usually tell the story
    fig2, axes2 = plt.subplots(1, 2, figsize=(12, 4), constrained_layout=True)
    plot_box(axes2[0], series_m1, "overhead_included_ms", "Overhead distribution (included)", "ms")
    plot_box(axes2[1], series_m1, "overhead_finalized_ms", "Overhead distribution (finalized)", "ms")

    print("\nMetric 1 summary (submit→included, seconds)")
    print(summarize(series_m1, "included_s").to_string(index=False))
    print("\nMetric 1 summary (submit→finalized, seconds)")
    print(summarize(series_m1, "finalized_s").to_string(index=False))
    print("\nMetric 1 summary (overhead→included, ms)")
    print(summarize(series_m1, "overhead_included_ms").to_string(index=False))
    print("\nMetric 1 summary (overhead→finalized, ms)")
    print(summarize(series_m1, "overhead_finalized_ms").to_string(index=False))


# metric 2 (key generation)
def plot_metric2_keygen(files: dict):
    series = load_algo_csvs(files)
    for alg, df in series.items():
        ensure_cols(df, {"iter", "keygen_us"}, f"Metric2 {alg}")

    series_k = {}
    for alg, df in series.items():
        out = df.copy()
        out["keygen_ms"] = out["keygen_us"] / 1000.0
        series_k[alg] = out

    fig, axes = plt.subplots(1, 2, figsize=(12, 4), constrained_layout=True)

    # --- OLD time-series (kept, but commented) ---
    # plot_timeseries(axes[0], series_k, "iter", "keygen_ms", "Metric 2 — Key generation", "ms")

    # --- NEW: mean bar chart ---
    plot_bar_mean(axes[0], series_k, "keygen_ms", "Metric 2 — Key generation (mean)", "ms")
    plot_box(axes[1], series_k, "keygen_ms", "Keygen distribution", "ms")

    print("\nMetric 2 summary (keygen, ms)")
    print(summarize(series_k, "keygen_ms").to_string(index=False))


# metric 3 (signing)
def plot_metric3_signing(files: dict):
    series = load_algo_csvs(files)
    for alg, df in series.items():
        ensure_cols(df, {"iter", "sign_us"}, f"Metric3 {alg}")

    series_s = {}
    for alg, df in series.items():
        out = df.copy()
        out["sign_ms"] = out["sign_us"] / 1000.0
        series_s[alg] = out

    fig, axes = plt.subplots(1, 2, figsize=(12, 4), constrained_layout=True)

    # --- OLD time-series (kept, but commented) ---
    # plot_timeseries(axes[0], series_s, "iter", "sign_ms", "Metric 3 — Transaction signing", "ms")

    # --- NEW: mean bar chart ---
    plot_bar_mean(axes[0], series_s, "sign_ms", "Metric 3 — Transaction signing (mean)", "ms")
    plot_box(axes[1], series_s, "sign_ms", "Signing distribution", "ms")

    print("\nMetric 3 summary (signing, ms)")
    print(summarize(series_s, "sign_ms").to_string(index=False))

'''
# metric 4 (runtime verification)
def plot_metric4_verification(path: str):
    df = read_csv(path)
    required = {"algorithm", "iteration", "time_us_median_slopes"}
    ensure_cols(df, required, f"Metric4 {path}")

    series_v = {}
    for algo in df["algorithm"].unique():
        sub = df[df["algorithm"] == algo].copy()
        sub["time_ms"] = sub["time_us_median_slopes"] / 1000.0
        series_v[algo] = sub

    fig, axes = plt.subplots(1, 2, figsize=(12, 4), constrained_layout=True)

    # --- OLD time-series (kept, but commented) ---
    # plot_timeseries(axes[0], series_v, "iteration", "time_ms",
    #                 "Metric 4 — Runtime signature verification (median slopes)", "ms")

    # --- NEW: mean bar chart ---
    plot_bar_mean(axes[0], series_v, "time_ms",
                  "Metric 4 — Runtime signature verification (mean)", "ms")
    plot_box(axes[1], series_v, "time_ms", "Verification distribution", "ms")

    print("\nMetric 4 summary (verification, ms)")
    print(summarize(series_v, "time_ms").to_string(index=False))
'''

def main():
    set_plot_style()

    plot_metric1_ledger(FILES_METRIC1, smooth_window=0)
    plot_metric2_keygen(FILES_METRIC2)
    plot_metric3_signing(FILES_METRIC3)
    # plot_metric4_verification(FILE_METRIC4)

    plt.show()


if __name__ == "__main__":
    main()