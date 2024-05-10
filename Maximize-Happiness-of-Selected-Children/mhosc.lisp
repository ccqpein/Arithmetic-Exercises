(defun maximum-happiness-sum (happiness k)
  (let ((happiness (sort happiness #'>))
        (result 0))
    (loop for ind from 0
          for h in (subseq happiness 0 k)
          when (< h ind)
            return result
          do (incf result (- h ind))
          finally (return result))
    ))
