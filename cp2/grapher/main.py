import csv
from matplotlib import pyplot as plt

FILE = '../report/data.csv'

reader = csv.reader(open(FILE))

entries = list(reader)
print(entries)

experiment_lengths = set(map(lambda x: x[1], entries))
print(experiment_lengths)
ops = set(map(lambda x: x[0], entries))
num_lengths = set(map(lambda x: x[2], entries))

for op in ops:
    fig, ax = plt.subplots()
    ax.minorticks_on()
    ax.set_title(f'Операція mod {op}')
    ax.set_xlabel('біт')
    for el in sorted(experiment_lengths):
        print(f'El = {el}')
        experiment = list(map(lambda x: (int(x[2]), int(x[3])), filter(lambda x: x[0] == op and x[1] == el, entries)))
        print(f'Ex: {experiment}')

        if el == '10000':
            color = 'b'
        elif el == '100000':
            color = 'g'
        else:
            color = 'r'

        print('will scatter: ', *zip(*experiment))
        ax.scatter(*zip(*experiment), label=f'{el} вим.', c=color)
        # ax.set(ylim=(IOC_RANDOM, 0.06))

    ax.set_ylabel('нс')


    handles, labels = ax.get_legend_handles_labels()
    ax.legend(handles, labels)
    plt.show()

    optext = None
    match op:
        case '+':
            optext = 'add'
        case '*':
            optext = 'mul'
        case '-':
            optext = 'sub'

    fig.savefig(f'../report/{el}{optext}.png')
