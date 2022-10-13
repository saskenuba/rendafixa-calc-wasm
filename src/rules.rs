use rust_decimal::{Decimal, MathematicalOps};
use rust_decimal_macros::dec;

#[derive(Debug, Clone, PartialEq)]
pub struct GrossNet {
    pub gross_amount: Decimal,
    pub net_amount: Decimal,
}

/// Returns the net amount after IR is deducted from `valor_rentabilidade`.
pub fn rule_rendafixa_net(valor_rentabilidade: Decimal, periodo_meses: i32) -> Decimal {
    let incidencia_ir = match periodo_meses {
        x if periodo_meses > 24 => dec!(0.15),
        x if periodo_meses <= 24 && periodo_meses > 12 => dec!(0.175),
        x if periodo_meses <= 12 && periodo_meses > 6 => dec!(0.205),
        x if periodo_meses <= 6 => dec!(0.225),
        _ => unreachable!(),
    };

    valor_rentabilidade * incidencia_ir
}

pub fn rule_cdb(
    valor_inicial: Decimal,
    taxa_cdi: Decimal,
    rentabilidade_cdi_no_periodo: Decimal,
    periodo_meses: i32,
) -> GrossNet {
    let gross_gain = valor_inicial * (taxa_cdi * rentabilidade_cdi_no_periodo);
    let net_gain = rule_rendafixa_net(gross_gain, periodo_meses);

    GrossNet {
        gross_amount: gross_gain,
        net_amount: net_gain,
    }
}

pub fn rule_compound(initial_amount: Decimal, taxa_juros: Decimal, periodo_meses: i32) -> Decimal {
    initial_amount * (dec!(1) + taxa_juros).powi(periodo_meses.into())
}

/// Returns the percentage that should be added to value birthday.
pub fn rule_poupanca(
    initial_amount: Decimal,
    taxa_selic: Decimal,
    taxa_referencial: Decimal,
) -> GrossNet {
    let res = if taxa_selic < dec!(0.085) {
        taxa_selic * dec!(0.7) + taxa_referencial
    } else {
        dec!(0.05) + taxa_referencial
    };

    let net_gain = initial_amount * (dec!(1) + res);

    // poupanca não paga IR
    GrossNet {
        gross_amount: net_gain,
        net_amount: net_gain,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    #[test]
    fn sample_poup() {
        let res = rule_poupanca(dec!(10), dec!(0.1375), dec!(0.01781));
        assert_eq!(res, dec!(10.6781));
    }

    #[test]
    fn sample_compound() {
        let res = rule_compound(dec!(10), dec!(0.10), 2);
        assert_eq!(res, dec!(12.1));
    }

    #[test]
    fn sample_rendafixa() {
        let res = rule_rendafixa_net(dec!(2.16), 2);
        assert_eq!(res, dec!(0.486));
    }
}
