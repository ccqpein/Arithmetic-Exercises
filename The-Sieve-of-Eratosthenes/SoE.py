# interater version


class myClass():
    prime_numbers = []
    index = 0
    count = 0

    def __init__(self, num):
        if num <= 0:
            raise RuntimeError("Not positive integer")
        elif num == 1:
            raise RuntimeError("equal 1")
        elif type(num) != int:
            raise RuntimeError("not integer")

        temp_list = [i for i in range(2, num + 1)]
        ind = 0
        while True:
            for i in temp_list[ind + 1:]:
                if i % temp_list[ind] == 0:
                    temp_list.remove(i)
            if temp_list[ind] ** 2 >= temp_list[-1]:
                break
            ind += 1
        self.prime_numbers = temp_list
        self.count = len(self.prime_numbers)

    def __iter__(self):
        return self

    def __next__(self):
        if self.count == 0:
            raise StopIteration
        else:
            self.index += 1
            self.count -= 1
            return self.prime_numbers[self.index - 1]


for i in myClass(20):
    print(i)
