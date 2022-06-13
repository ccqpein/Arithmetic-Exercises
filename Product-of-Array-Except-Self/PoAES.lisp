(defun product-except-self (nums)
  (let ((result (make-list (length nums) :initial-element 1))
        pre
        (post 1))
    (loop for i from 1 below (length nums)
          do (setf pre (nth (1- i) result)
                   (nth i result) (* pre (nth (1- i) nums))))

    (loop for i from (- (length nums) 2) downto 0
          do (setf post (* post (nth (1+ i) nums))
                   (nth i result) (* (nth i result) post)))
    result))
