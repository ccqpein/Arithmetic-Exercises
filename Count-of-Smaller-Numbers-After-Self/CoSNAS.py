def countSmaller(nums):
    rlist = []
    while(nums[0]):
        try:
            nums[1]
        except IndexError:
            time =0
            rlist.append(time)
            break
        else:
            time = 0
            a = nums[0]
            print(a)
            nums = nums[1:]
            print(nums)
            for i in nums:
                if i <a:
                    time +=1
            rlist.append(time)

    return rlist
