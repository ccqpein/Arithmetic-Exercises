class Solution:

    def killProcess(self, pid, ppid, kill):
        """
        :type pid: List[int]
        :type ppid: List[int]
        :type kill: int
        :rtype: List[int]
        """

        tempDict = {}

        for i in range(len(ppid)):
            try:
                if ppid[i]:
                    tempDict[ppid[i]].append(pid[i])
            except:
                tempDict[ppid[i]] = [pid[i]]

        rest, stack = [], [kill]
        while stack:
            this = stack.pop()
            rest.append(this)
            try:
                stack.extend(tempDict[this])
            except:
                continue

        return rest
