(defun rob (nums)
  (labels ((inner-func (nums start len table)
             (let (v p result)
               (multiple-value-setq (v p) (gethash start table))
               (cond (p (setf result v))

                     ((eql start (1- len))
                      (setf (gethash start table) (nth start nums)
                            result (nth start nums)))

                     ((eql start len)
                      (setf (gethash start table) 0
                            result 0))

                     (t
                      (let (a)
                        (setf a (max (+ (nth start nums)
                                        (inner-func nums (+ 2 start) len table))
                                     (apply #'max
                                            (loop
                                              for x from (1+ start) to (1- len)
                                              collect (inner-func nums x len table))))
                              (gethash start table) a
                              result a))))
               result)))
    (let ((table (make-hash-table))
          (len (length nums)))
      (inner-func nums 0 len table))))

(defun rob2 (nums
             &optional (start 0) len table
             &aux ;; aux will rewrite optional
               (len (if (not len) (length nums) len))
               (table (if (not table) (make-hash-table) table)))
  (let (v p result)
    (multiple-value-setq (v p) (gethash start table))
    (cond (p (setf result v))

          ((eql start (1- len))
           (setf (gethash start table) (nth start nums)
                 result (nth start nums)))

          ((eql start len)
           (setf (gethash start table) 0
                 result 0))

          (t
           (let (a)
             (setf a (max (+ (nth start nums)
                             (rob2 nums (+ 2 start) len table))
                          (apply #'max
                                 (loop
                                   for x from (1+ start) to (1- len)
                                   collect (rob2 nums x len table))))
                   (gethash start table) a
                   result a))))
    result))

(defun main ()
  (eql (rob '(2 7 9 3 1)) 12)
  (eql (rob '(1 2 3 1)) 4)
  (eql (rob '(183 219 57 193 94 233 202 154 65 240 97 234 100 249 186 66 90 238 168 128 177 235 50 81 185 165 217 207 88 80 112 78 135 62 228 247 211)) 3365)
  (eql (rob '(9 297 196 336 435 2 343 159 146 359 45 470 265 131 17 271 74 242 448 402 55 423 414 240 430 135 322 468 422 351 441 463 30 399 132 439 463 260 399 32 374 383 276 166 104 315 314 445 458 422 104 251 382 230 484 127 355 332 317 362 257 493 474 401 40 93 433 464 136 342 98 159 223 110 89 47 53 179 219 314 486 301 307 453 37 366 334 355 26 484 124 408 346 133 420 280 124 210 358 140 9 297 196 336 435 2 343 159 146 359 45 470 265 131 17 271 74 242 448 402 55 423 414 240 430 135 322 468 422 351 441 463 30 399 132 439 463 260 399 32 374 383 276 166 104 315 314 445 458 422 104 251 382 230 484 127 355 332 317 362 257 493 474 401 40 93 433 464 136 342 98 159 223 110 89 47 53 179 219 314 486 301 307 453 37 366 334 355 26 484 124 408 346 133 420 280 124 210 358 140 9 297 196 336 435 2 343 159 146 359 45 470 265 131 17 271 74 242 448 402 55 423 414 240 430 135 322 468 422 351 441 463 30 399 132 439 463 260 399 32 374 383 276 166 104 315 314 445 458 422 104 251 382 230 484 127 355 332 317 362 257 493 474 401 40 93 433 464 136 342 98 159 223 110 89 47 53 179 219 314 486 301 307 453 37 366 334 355 26 484 124 408 346 133 420 280 124 210 358 140)) 46896))
