;; СДЕЛАТЬ: расширения для vim
(defconst синтаксическая-таблица-режима-хуя
  (with-syntax-table (copy-syntax-table)
    ;; Комментарии в стиле Си/Си++
	(modify-syntax-entry ?/ ". 124b")
	(modify-syntax-entry ?* ". 23")
	(modify-syntax-entry ?\n "> b")

    ;; Строковые литералы
    (modify-syntax-entry ?« "(»")
    (modify-syntax-entry ?» ")«")
    (modify-syntax-entry ?\" ".")

    (syntax-table))
  "Синтаксическая таблица для `режим-хуя'.")

(eval-and-compile
  (defconst ключевые-слова-хуя
    '("пер" "про" "конст" "пока" "нч" "кц" "для"
      "если" "то" "иначе" "вернуть" "замкнуть" "пропустить"
      "структ" "союз" "как" "вкл" "внешняя" "библ" "или"
      "и" "истина" "ложь" "лбс" "пбс" "ост" "вилка" "когда" "любое"
      "либо")))

(defun строковый-литерал-хуя (придел)
  (while (and (< (point) придел)
              (not (or (eq (char-after) ?«) (eq (char-after) ?\"))))
    (forward-char))
  (cond
   ((eq (char-after) ?«)
    (let ((начало (point)))
      (forward-char)
      (let ((вложенность 1))
        (while (and (< (point) придел) (> вложенность 0))
          (cond
           ;; СДЕЛАТЬ: поддержка экранирования внутри «ёлочных» литералов
           ((eq (char-after) ?«) (incf вложенность))
           ((eq (char-after) ?») (decf вложенность)))
          (forward-char)))
      (set-match-data (list начало (point)) t)
      (point)))
   ((eq (char-after) ?\")
    (let ((начало (point)))
      (forward-char)
      (let ((кончили nil))
        (while (and (< (point) придел) (not кончили))
          ;; СДЕЛАТЬ: поддержка экранирования внутри "лапочных" литералов
          (when (eq (char-after) ?\")
            (setq кончили t))
          (forward-char)))
      (set-match-data (list начало (point)) t)
      (point)))))

;; СДЕЛАТЬ: подсветка многострочных «ёлочных» литералов.
;; Она почему-то не работает из коробки
(defconst подсветка-хуя
  `((строковый-литерал-хуя . font-lock-string-face)
    (,(regexp-opt ключевые-слова-хуя 'symbols) . font-lock-keyword-face)))

;;;###autoload
(define-derived-mode режим-хуя prog-mode "хуя"
  "Основной режим (анг. Major mode) для редактирования исходных файлов на языке ХУЯ."
  :syntax-table синтаксическая-таблица-режима-хуя
  (setq font-lock-defaults '(подсветка-хуя))
  (setq-local comment-start "// "))

;;;###autoload
(add-to-list 'auto-mode-alist '("\\.хуя\\'" . режим-хуя))
