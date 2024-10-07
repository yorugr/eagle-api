use serde::Serializer;

pub(crate) fn comma_separated<S>(v: &[String], s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    s.serialize_str(v.join(",").as_str())
}
