;; load files into repl
(def load-file
  (λ [filename]
     (eval (read (str "(do " (slurp filename) ")")))))

;; lexical scoping with closures
((λ [btree]
    (do
      ;; access values of map
      (println (:a btree))
      (println (str (type btree) " " btree-map))))
 (conj {:a 1 :b 2} {:a 2 :c 3}))

;; hello world function
(def hi-world (λ [name] (println (str "hello " name))))

;; map values are evaluated each time they are used
(println {:a (+ 2/3 5.0)})
;; {:a (+, 2/3, 5.0, ), }
(:a {:a (+ 2/3 5.0)})
;; => 5.666666666666667


;; math
(+ 1 2 3 4)
;; => 10
(+ 2 2/3 -1.0)
;; => 1.6666666666666665
(* -1 1.21e10)
;; => -12100000000.0
(/ 3 2)
;; => 1.5
(mod 3 2)
;; => 1.0
(int 8/3)
;; => 3

;; loading file returns "return value"
;; empty string is falsy
(if ""
  (println "nil == true")
  "return value")
