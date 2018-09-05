class Solution:
    def findClosestElements(self, arr, k, x):
        """
        :type arr: List[int]
        :type k: int
        :type x: int
        :rtype: List[int]
        """
        arr_larger = [(i, e) for i, e in enumerate(arr) if e >= x][0]

        i = arr_larger[0]-k if arr_larger[0] - k >= 0 else 0
        j = arr_larger[0] + k if arr_larger[0] + k <= len(arr) else len(arr)

        inner_arr = [e for e in arr[i:j]]
        result = sorted(inner_arr, key=lambda v: abs(v-x))

        return sorted(result[:k])

# a.findClosestElements([1,2,3,4,5], k=4, x=3) => [1,2,3,4]
# a.findClosestElements([0,0,0,1,3,5,6,7,8,8], k=2, x=2) => [1,3]
