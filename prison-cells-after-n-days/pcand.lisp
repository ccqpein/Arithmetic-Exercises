(ql:quickload "alexandria")

(defun prison-after-n-days (cells n)
  (let ((cache (make-hash-table :test 'equal))
        (cache2 (make-hash-table :test 'equal)))
    (loop for nn from 1 to n
          do
             (setf cells (one-step cells))
             (alexandria:if-let ((nnn (gethash cells cache)))
               (return-from prison-after-n-days
                 (gethash
                  (+ (mod (- n nnn)
                          (- nn nnn))
                     nnn)
                  cache2)))
             (setf (gethash cells cache) nn
                   (gethash nn cache2) cells)
          )
    cells))

(defun one-step (cells)
  (append '(0)
          (loop for ind from 1 below (1- (length cells))
                collect (if (= (nth (1- ind) cells) (nth (1+ ind) cells))
                            1
                            0))
          '(0)))

(defun main ()
  ;; Input: cells = [0,1,0,1,1,0,0,1], n = 7
  ;; Output: [0,0,1,1,0,0,0,0]
  (assert (equal (prison-after-n-days '(0 1 0 1 1 0 0 1) 7)
                 '(0 0 1 1 0 0 0 0)))
  
  ;; Input: cells = [1,0,0,1,0,0,1,0], n = 1000000000
  ;; Output: [0,0,1,1,1,1,1,0]
  (assert (equal (prison-after-n-days '(1 0 0 1 0 0 1 0) 1000000000)
                 '(0 0 1 1 1 1 1 0))))
