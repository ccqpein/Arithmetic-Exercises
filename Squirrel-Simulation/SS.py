class Solution:

    def minDistance(self, height, width, tree, squirrel, nuts):
        """
        :type height: int
        :type width: int
        :type tree: List[int]
        :type squirrel: List[int]
        :type nuts: List[List[int]]
        :rtype: int
        """
        def dist(p1, p2):
            return abs(p1[0] - p2[0]) + abs(p1[1] - p2[1])

        minNutDist = dist(nuts[0], squirrel) - dist(nuts[0], tree)
        nuts2tree = 0
        for nut in nuts:
            nuts2tree += 2 * dist(nut, tree)
            if dist(nut, squirrel) - dist(nut, tree) <= minNutDist:
                minNutDist = dist(nut, squirrel) - dist(nut, tree)

        return nuts2tree + minNutDist
