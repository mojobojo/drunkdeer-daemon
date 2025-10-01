// NOTE: the real structure makes this a 64 byte command however in order to not get confused while
//	reverse engineering the js for the web driver I decided to keep the offsets the same.
pub struct DataPacket {
    pub report_id: u8,
    pub command: u8,
    pub daata: [u8; 62],
}
