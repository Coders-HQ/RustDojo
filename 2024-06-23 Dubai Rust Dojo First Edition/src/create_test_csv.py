# Generate a random csv file with date,open,high,low,close,volume columns with random values with a specific seed and a givin number of rows

import random
import datetime

def generate_random_csv(num_rows, seed=42, filename='random_data.csv'):
    random.seed(seed)

    with open(filename, 'w') as f:
        f.write('date,open,high,low,close,volume\n')

        for _ in range(num_rows):
            date = (datetime.datetime.now() + datetime.timedelta(days=1)).strftime('%Y-%m-%d')
            open_val = random.uniform(1, 100)
            high_val = random.uniform(1, 100)
            low_val = random.uniform(1, 100)
            close_val = random.uniform(1, 100)
            volume_val = random.randint(1, 1000000)

            line = f'{date},{open_val},{high_val},{low_val},{close_val},{volume_val}\n'
            f.write(line)

# Usage
generate_random_csv(50000000, seed=42, filename='large_file50m.csv')