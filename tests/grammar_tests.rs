use pest::Parser;
use anyhow::anyhow;
use pest_practice::*;

#[test]
fn basic_test() -> anyhow::Result<()>
{
    let a = Grammar::parse(Rule::field, "-273.45")?.next().ok_or_else(|| anyhow!("No pair"))?;

    assert_eq!(a.as_str(), "-273.45");
    assert_eq!(a.as_span().start(), 0);
    assert_eq!(a.as_span().end(), 7);

    let b = Grammar::parse(Rule::field, "");
    assert!(b.is_err());

    let c = Grammar::parse(Rule::field, "A");
    assert!(c.is_err());

    Ok(())
}

#[test]
fn record_test() -> anyhow::Result<()>
{
    let a = Grammar::parse(Rule::record, "-273.45,12")?.next().ok_or_else(|| anyhow!("No pair"))?;

    assert_eq!(a.as_str(), "-273.45,12");
    assert_eq!(a.as_span().start(), 0);
    assert_eq!(a.as_span().end(), 10);

    let b = Grammar::parse(Rule::record, "");
    assert!(b.is_err());

    Ok(())
}

#[test]
fn file_test_single_record() -> anyhow::Result<()>
{
    let a = Grammar::parse(Rule::file, "-273.45,12\n")?.next().ok_or_else(|| anyhow!("No pair"))?;

    assert_eq!(a.as_str(), "-273.45,12\n");

    Ok(())
}

#[test]
fn file_test_multiple_records() -> anyhow::Result<()>
{
    let a = Grammar::parse(Rule::file, "-273.45,12\n-273.45,12\n")?.next().ok_or_else(|| anyhow!("No pair"))?;

    assert_eq!(a.as_str(), "-273.45,12\n-273.45,12\n");

    Ok(())
}

#[test]
fn file_test_invalid_records() -> anyhow::Result<()>
{
    let a = Grammar::parse(Rule::file, "-273.45 12");
    assert!(a.is_err());

    let b = Grammar::parse(Rule::file, "-273.45,12");
    assert!(b.is_err());

    Ok(())
}
