(defun truncate-sentence (str k)
  (subseq str
          0
          (loop
            with cc = 0
            for c across str

            if (char= c #\ )
              do (decf k)
            end

            if (= 0 k)
              return cc
            end
            do (incf cc)
            )))
