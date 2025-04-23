(ql:quickload "SPLIT-SEQUENCE")

(defun build-tree (preorder inorder)
  (if (not inorder) (return-from build-tree nil))

  (let* ((root (car preorder))
         (l-and-r (split inorder root))
         (left (first l-and-r))
         (right (second l-and-r)))
    (append (list root) (list left) (list right))))

(defun split (inorder k)
  (split-sequence:split-sequence-if (lambda (v) (= v k)) inorder))
