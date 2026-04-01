#[derive(Debug)]
pub enum Expr {
    Number(f64),
    Binary {
        left: Box<Expr>,
        op: BinaryOp,
        right: Box<Expr>,
    },
}

#[derive(Debug)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug)]
pub enum EvalError {
    DivisionByZero,
}

impl Expr {
    pub fn eval(&self) -> Result<f64, EvalError> {
        match self {
            Expr::Number(n) => Ok(*n),
            Expr::Binary { left, op, right } => {
                let l = left.eval()?;
                let r = right.eval()?;

                match op {
                    BinaryOp::Add => Ok(l + r),
                    BinaryOp::Sub => Ok(l - r),
                    BinaryOp::Mul => Ok(l * r),
                    BinaryOp::Div => {
                        if r == 0.0 {
                            Err(EvalError::DivisionByZero)
                        } else {
                            Ok(l / r)
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number() {
        let expr = Expr::Number(42.0);
        assert_eq!(expr.eval().unwrap(), 42.0);
    }

    #[test]
    fn test_addition() {
        let expr = Expr::Binary {
            left: Box::new(Expr::Number(2.0)),
            op: BinaryOp::Add,
            right: Box::new(Expr::Number(3.0)),
        };

        assert_eq!(expr.eval().unwrap(), 5.0);
    }

    #[test]
    fn test_subtraction() {
        let expr = Expr::Binary {
            left: Box::new(Expr::Number(10.0)),
            op: BinaryOp::Sub,
            right: Box::new(Expr::Number(4.0)),
        };

        assert_eq!(expr.eval().unwrap(), 6.0);
    }

    #[test]
    fn test_multiplication() {
        let expr = Expr::Binary {
            left: Box::new(Expr::Number(3.0)),
            op: BinaryOp::Mul,
            right: Box::new(Expr::Number(4.0)),
        };

        assert_eq!(expr.eval().unwrap(), 12.0);
    }

    #[test]
    fn test_division() {
        let expr = Expr::Binary {
            left: Box::new(Expr::Number(8.0)),
            op: BinaryOp::Div,
            right: Box::new(Expr::Number(2.0)),
        };

        assert_eq!(expr.eval().unwrap(), 4.0);
    }

    #[test]
    fn test_division_by_zero() {
        let expr = Expr::Binary {
            left: Box::new(Expr::Number(1.0)),
            op: BinaryOp::Div,
            right: Box::new(Expr::Number(0.0)),
        };

        let result = expr.eval();
        assert!(matches!(result, Err(EvalError::DivisionByZero)));
    }
}
