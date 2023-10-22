import sys
import time

import matplotlib.pyplot as plt
import seaborn as sns
import pandas as pd

def plot(data, path):
    plt.figure(figsize=(10, 6), dpi=100)
    sns.set_style("whitegrid")

    sns_plot = sns.lineplot(x="generation", y="fitness", hue="type", data=data)

    plt.title('Convergence', fontsize=16)
    plt.xlabel('Generation', fontsize=14)
    plt.ylabel('Fitness (Higher is better)', fontsize=14)

    plt.xticks(fontsize=12)
    plt.yticks(fontsize=12)

    plt.legend(loc='lower right', fontsize=12)

    fig = sns_plot.get_figure()
    fig.savefig(path)


if __name__ == '__main__':
    name = sys.argv[1]
    runs = int(sys.argv[2]) if len(sys.argv) > 2 else 1

    dataframes = []
    for i in range(1, runs + 1):
        input_path = f"out/{name}_run_{i}.csv"
        df = pd.read_csv(input_path, header=None)
        df["avg"] = df.mean(axis=1)
        df["best"] = df.max(axis=1)
        dataframes.append(df)
        
    max_len = max([len(df) for df in dataframes])

    avgs = []
    bests = []
    for i in range(len(dataframes)):
        avg = dataframes[i]["avg"]
        avg_extended = avg.reindex(range(max_len), fill_value=1.0)
        avgs.append(avg_extended)
        
        best = dataframes[i]["best"]
        best_extended = best.reindex(range(max_len), fill_value=1.0)
        bests.append(best_extended)

    avgs_df = pd.concat(avgs, axis=0)
    bests_df = pd.concat(bests, axis=0)
    data_df = pd.concat([avgs_df, bests_df], keys=["Average", "Best"])
    data_df = data_df.reset_index()
    data_df.columns = ['type', 'generation', 'fitness']

    start = time.time()
    plot(data_df, f"out/{name}_convergence.png")
    end = time.time()
    
    print(f"Time: {end - start}")
