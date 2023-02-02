import random

values_1000 = random.sample(range(-1000,1000),1000)
values_100 = random.sample(range(-100,100),100)

with open('values1000.csv', 'w') as f:
    for value in values_1000:
        f.write(str(value)+ ',')
    f.close()

with open('values100.csv', 'w') as f:
    for value in values_100:
        f.write(str(value) + ',')
    f.close()
