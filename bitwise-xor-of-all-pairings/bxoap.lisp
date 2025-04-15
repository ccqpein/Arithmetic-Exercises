(defun xor-all-nums (nums1 nums2)
  (labels ((xor-times (v times)
             (if (= 0 (mod times 2)) 0 v)))
    (logxor
     (reduce #'logxor (loop for n in nums1
                            collect (xor-times n (length nums2))))
     (reduce #'logxor (loop for n in nums2
                            collect (xor-times n (length nums1)))))))

(xor-all-nums '(2 1 3) '(10 2 5 0))
