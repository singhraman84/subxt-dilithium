import pandas as pd
import matplotlib.pyplot as plt
import numpy as np
from pathlib import Path

US_TO_S = 1_000_000.0
US_TO_MS = 1_000.0

BASE_DIR = Path(__file__).resolve().parent
DIR = BASE_DIR / "csv-files"

FILES_METRIC1 = {
    "sr25519": "sr25519_measure_transaction_latency.csv",
    "ecdsa": "ecdsa_measure_transaction_latency.csv",
    "ml_dsa_44": "ml_dsa_44_measure_transaction_latency.csv",
    "ml_dsa_65": "ml_dsa_65_measure_transaction_latency.csv",
    "ml_dsa_87": "ml_dsa_87_measure_transaction_latency.csv",
    # instant seal
    "sr25519_instant": "sr25519_measure_transaction_latency_instant.csv",
    "ecdsa_instant": "ecdsa_measure_transaction_latency_instant.csv",
    "ml_dsa_44_instant": "ml_dsa_44_measure_transaction_latency_instant.csv",
    "ml_dsa_65_instant": "ml_dsa_65_measure_transaction_latency_instant.csv",
    "ml_dsa_87_instant": "ml_dsa_87_measure_transaction_latency_instant.csv",
}

FILES_METRIC2 = {
    "sr25519": "sr25519_keygen.csv",
    "ecdsa": "ecdsa_keygen.csv",
    "ml_dsa_44": "ml_dsa_44_keygen.csv",
    "ml_dsa_65": "ml_dsa_65_keygen.csv",
    "ml_dsa_87": "ml_dsa_87_keygen.csv",
}

FILES_METRIC3 = {
    "sr25519": "sr25519_sign_latency.csv",
    "ecdsa": "ecdsa_sign_latency.csv",
    "ml_dsa_44": "ml_dsa_44_sign_latency.csv",
    "ml_dsa_65": "ml_dsa_65_sign_latency.csv",
    "ml_dsa_87": "ml_dsa_87_sign_latency.csv",
}

FILES_METRIC4 = {
    "sr25519": "sr25519_verify_time.csv",
    "ecdsa": "ecdsa_verify_time.csv",
    "ml_dsa_44": "ml_dsa_44_verify_time.csv",
    "ml_dsa_65": "ml_dsa_65_verify_time.csv",
    "ml_dsa_87": "ml_dsa_87_verify_time.csv",
}

FILES_METRIC5 = {
    "sr25519": "sr25519_xt_size_weight.csv",
    "ecdsa": "ecdsa_xt_size_weight.csv",
    "ml_dsa_44": "ml_dsa_44_xt_size_weight.csv",
    "ml_dsa_65": "ml_dsa_65_xt_size_weight.csv",
    "ml_dsa_87": "ml_dsa_87_xt_size_weight.csv",
}

ORDER = ["sr25519", "ecdsa", "ml_dsa_44", "ml_dsa_65", "ml_dsa_87"]

DISPLAY_NAME = {
    "sr25519": "sr25519",
    "ecdsa": "ECDSA",
    "ml_dsa_44": "ML-DSA-44",
    "ml_dsa_65": "ML-DSA-65",
    "ml_dsa_87": "ML-DSA-87",
}

ALG_COLOR = {
    "sr25519": "#1f77b4",
    "ecdsa": "#ff7f0e",
    "ml_dsa_44": "#2ca02c",
    "ml_dsa_65": "#d62728",
    "ml_dsa_87": "#9467bd",
}


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
        "font.size": 10,
    })


def read_csv(path: str) -> pd.DataFrame:
    full_path = DIR / path
    if not full_path.exists():
        raise FileNotFoundError(f"Missing file: {full_path}")
    return pd.read_csv(full_path)


def load_algo_csvs(files: dict) -> dict:
    return {algo: read_csv(path) for algo, path in files.items()}


def ensure_cols(df: pd.DataFrame, required: set, label: str):
    missing = sorted(required - set(df.columns))
    if missing:
        raise ValueError(f"{label}: missing columns {missing}")


def reorder_series(series: dict) -> dict:
    return {k: series[k] for k in ORDER if k in series}


def base_alg(name: str) -> str:
    return name.replace("_instant", "")


def is_instant(name: str) -> bool:
    return name.endswith("_instant")


def split_metric1(files: dict):
    normal = {base_alg(k): v for k, v in files.items() if not is_instant(k)}
    instant = {base_alg(k): v for k, v in files.items() if is_instant(k)}
    return normal, instant


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
    return pd.DataFrame(rows).sort_values("mean")


def add_bar_labels(ax, bars, fmt="{:.2f}", rotation=90, fontsize=8):
    y_min, y_max = ax.get_ylim()
    offset = (y_max - y_min) * 0.01

    for bar in bars:
        h = bar.get_height()
        if np.isnan(h):
            continue
        ax.text(
            bar.get_x() + bar.get_width() / 2,
            h + offset,
            fmt.format(h),
            ha="center",
            va="bottom",
            rotation=rotation,
            fontsize=fontsize,
        )


def plot_grouped_bar(ax, metrics, title, y_label, value_fmt="{:.2f}"):
    algs = [alg for alg in ORDER if alg in next(iter(metrics.values()))]
    metric_names = list(metrics.keys())

    x = np.arange(len(metric_names))
    width = 0.15

    offsets = np.linspace(-(len(algs) - 1) / 2, (len(algs) - 1) / 2, len(algs)) * width

    for i, alg in enumerate(algs):
        values = [metrics[m][alg] for m in metric_names]
        bars = ax.bar(
            x + offsets[i],
            values,
            width=width,
            label=DISPLAY_NAME.get(alg, alg),
            color=ALG_COLOR.get(alg),
        )
        add_bar_labels(ax, bars, fmt=value_fmt)

    ax.set_xticks(x)
    ax.set_xticklabels(metric_names)
    ax.set_title(title)
    ax.set_ylabel(y_label)
    ax.legend(title="Algorithm")


def get_metric1_instant_means(files: dict):
    _, instant_files = split_metric1(files)
    series = reorder_series(load_algo_csvs(instant_files))

    required = {"iter", "included_us", "finalized_us", "all_included_us", "all_finalized_us"}
    for alg, df in series.items():
        ensure_cols(df, required, f"Metric1 Instant {alg}")

    out = {}
    for alg, df in series.items():
        out[alg] = {
            "included_ms": float((df["included_us"] / US_TO_MS).mean()),
            "finalized_ms": float((df["finalized_us"] / US_TO_MS).mean()),
            "all_included_ms": float((df["all_included_us"] / US_TO_MS).mean()),
            "all_finalized_ms": float((df["all_finalized_us"] / US_TO_MS).mean()),
        }
    return out


def get_metric2_keygen_means(files: dict):
    series = reorder_series(load_algo_csvs(files))
    for alg, df in series.items():
        ensure_cols(df, {"iter", "keygen_us"}, f"Metric2 {alg}")
    return {
        alg: float((df["keygen_us"] / US_TO_MS).mean())
        for alg, df in series.items()
    }


def get_metric3_signing_means(files: dict):
    series = reorder_series(load_algo_csvs(files))
    for alg, df in series.items():
        ensure_cols(df, {"iter", "sign_us"}, f"Metric3 {alg}")
    return {
        alg: float((df["sign_us"] / US_TO_MS).mean())
        for alg, df in series.items()
    }


def get_metric3_signing_throughput(files: dict):
    series = reorder_series(load_algo_csvs(files))
    for alg, df in series.items():
        ensure_cols(df, {"iter", "sign_us"}, f"Metric3 {alg}")
    return {
        alg: float((US_TO_S / df["sign_us"]).mean())
        for alg, df in series.items()
    }


def get_metric4_verify_means(files: dict):
    series = reorder_series(load_algo_csvs(files))
    for alg, df in series.items():
        ensure_cols(df, {"verification_time_us"}, f"Metric4 {alg}")
    return {
        alg: float((df["verification_time_us"] / US_TO_MS).mean())
        for alg, df in series.items()
    }


def get_metric4_verify_throughput(files: dict):
    series = reorder_series(load_algo_csvs(files))
    for alg, df in series.items():
        ensure_cols(df, {"verification_time_us"}, f"Metric4 {alg}")
    return {
        alg: float((US_TO_S / df["verification_time_us"]).mean())
        for alg, df in series.items()
    }


def get_metric5_size_weight_means(files: dict):
    series = reorder_series(load_algo_csvs(files))
    for alg, df in series.items():
        ensure_cols(df, {"iter", "xt_bytes", "xt_ref_time"}, f"Metric5 {alg}")

    size_means = {alg: float(df["xt_bytes"].mean()) for alg, df in series.items()}
    weight_means = {alg: float(df["xt_ref_time"].mean()) for alg, df in series.items()}
    return size_means, weight_means


def plot_latency_overview():
    instant = get_metric1_instant_means(FILES_METRIC1)
    keygen = get_metric2_keygen_means(FILES_METRIC2)
    signing = get_metric3_signing_means(FILES_METRIC3)
    verify = get_metric4_verify_means(FILES_METRIC4)

    metrics = {
        "Key Gen": keygen,
        "Signing": signing,
        "On-chain Verify": verify,
        "Tx Included": {alg: instant[alg]["all_included_ms"] for alg in instant},
        "Tx Finalized": {alg: instant[alg]["all_finalized_ms"] for alg in instant},
    }

    fig, ax = plt.subplots(figsize=(13, 6), constrained_layout=True)
    plot_grouped_bar(
        ax,
        metrics,
        "Performance Comparison of Cryptographic Algorithms",
        "Average Latency (ms)",
        value_fmt="{:.2f}",
    )

    print("\nLatency overview (ms)")
    rows = []
    for metric_name, metric_vals in metrics.items():
        for alg, val in metric_vals.items():
            rows.append({"metric": metric_name, "algorithm": alg, "mean": val})
    print(pd.DataFrame(rows).to_string(index=False))


def plot_throughput_overview():
    signing_tp = get_metric3_signing_throughput(FILES_METRIC3)
    verify_tp = get_metric4_verify_throughput(FILES_METRIC4)

    metrics = {
        "Signing Throughput": signing_tp,
        "Verification Throughput": verify_tp,
    }

    fig, ax = plt.subplots(figsize=(10, 6), constrained_layout=True)
    plot_grouped_bar(
        ax,
        metrics,
        "Cryptographic Throughput Comparison",
        "Operations per second",
        value_fmt="{:.0f}",
    )

    print("\nThroughput overview (ops/sec)")
    rows = []
    for metric_name, metric_vals in metrics.items():
        for alg, val in metric_vals.items():
            rows.append({"metric": metric_name, "algorithm": alg, "mean": val})
    print(pd.DataFrame(rows).to_string(index=False))


def plot_size_weight_overview():
    size_means, weight_means = get_metric5_size_weight_means(FILES_METRIC5)

    metrics = {
        "Extrinsic Size (bytes)": size_means,
        "Extrinsic Weight": weight_means,
    }

    fig, ax = plt.subplots(figsize=(11, 6), constrained_layout=True)
    plot_grouped_bar(
        ax,
        metrics,
        "Extrinsic Size and Weight Comparison",
        "Mean value",
        value_fmt="{:.0f}",
    )

    print("\nSize / weight overview")
    rows = []
    for metric_name, metric_vals in metrics.items():
        for alg, val in metric_vals.items():
            rows.append({"metric": metric_name, "algorithm": alg, "mean": val})
    print(pd.DataFrame(rows).to_string(index=False))


def main():
    set_plot_style()

    plot_latency_overview()
    plot_throughput_overview()
    plot_size_weight_overview()

    plt.show()


if __name__ == "__main__":
    main()