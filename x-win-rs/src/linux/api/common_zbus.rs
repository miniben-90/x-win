use zbus::{
  block_on,
  names::{BusName, InterfaceName, MemberName},
  zvariant::export::serde::Serialize,
  Connection, DynamicType, Error, Message, ObjectPath, Result,
};

pub fn call_method<'d, 'p, 'i, 'm, D, P, I, M, B>(
  destination: Option<D>,
  path: P,
  interface: Option<I>,
  method_name: M,
  body: &B,
) -> Result<Message>
where
  D: TryInto<BusName<'d>>,
  D::Error: Into<Error>,
  P: TryInto<ObjectPath<'p>>,
  P::Error: Into<Error>,
  I: TryInto<InterfaceName<'i>>,
  I::Error: Into<zbus::Error>,
  M: TryInto<MemberName<'m>>,
  M::Error: Into<zbus::Error>,
  B: Serialize + DynamicType,
{
  block_on(async {
    let session = Connection::session().await?;
    let message = session
      .call_method(destination, path, interface, method_name, body)
      .await?;
    Ok(message)
  })
}
