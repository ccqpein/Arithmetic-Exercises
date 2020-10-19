(defun new-binary-indexed-tree (ll)
  (append '(0)
          (loop
            for i from 0 below (length ll)
            for c = (nth i ll) and j = (1- i)
            do (loop
                 while (>= j (- (1+ i) (logand (1+ i) (- (1+ i)))))
                 do (incf c (nth j ll))
                 do (decf j))
            collect c
            )))

(defun update-binary-indexed-tree (bit ind delta)
  (assert (< ind (length bit)) (ind) "ind beyond the length")

  (loop
    with len = (length bit)
    while (< ind len)
    do (incf (nth ind bit) delta)
    do (incf ind (logand ind (- ind))))
  
  bit)

(defun sum-binary-indexed-tree (bit ind)
  (assert (< ind (length bit)) (ind) "ind beyond the length")

  (loop
    with result = 0
    while (> ind 0)
    do (incf result (nth ind bit))
    do (decf ind (logand ind (- ind)))
    finally (return result)
    ))

(defun reverse-pairs (nums)
  (let* ((new-nums (append nums (loop for num in nums collect (* 2 num))))
         (nums-set (sort (remove-duplicates new-nums) #'<))
         (position-plist (loop
                           for ind from 0
                           for num in nums-set
                           append (list num ind)))
         (bit (make-list (1+ (length nums-set)) :initial-element 0))
         )
    
    (loop
      with count = 0
      for i from (1- (length nums)) downto 0
      do (incf count (sum-binary-indexed-tree bit (getf position-plist (nth i nums))))
      do (update-binary-indexed-tree bit (1+ (getf position-plist (* 2 (nth i nums)))) 1)
      finally (return count)
      )))
