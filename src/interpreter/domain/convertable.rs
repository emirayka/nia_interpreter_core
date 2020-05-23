pub trait Convertable<NiaInterpreterType, NiaEventType> {
    fn to_nia_events_representation(&self) -> NiaEventType;
    fn from_nia_events_representation(
        value: &NiaEventType,
    ) -> NiaInterpreterType;
}
