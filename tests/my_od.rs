use canopen::objectdictionary::Variable;
use canopen::datatypes::*;

pub const OD_DUMMY0001: Variable =
    Variable::new(0x0001, 0x00, None);
pub const OD_DUMMY0002: Variable =
    Variable::new(0x0002, 0x00, None);
pub const OD_DEVICE_TYPE: Variable =
    Variable::new(0x1000, 0x00, None);
pub const OD_ERROR_REGISTER: Variable =
    Variable::new(0x1001, 0x00, None);
pub const OD_PRE_DEFINED_ERROR_FIELD_NUMBER_OF_ENTRIES: Variable =
    Variable::new(0x1003, 0x00, None);
pub const OD_PRE_DEFINED_ERROR_FIELD_PRE_DEFINED_ERROR_FIELD: Variable =
    Variable::new(0x1003, 0x01, None);
pub const OD_MANUFACTURER_DEVICE_NAME: Variable =
    Variable::new(0x1008, 0x00, Some(VISIBLE_STRING("TEST DEVICE")));
pub const OD_PRODUCER_HEARTBEAT_TIME: Variable =
    Variable::new(0x1017, 0x00, Some(UNSIGNED16(0x0)));
pub const OD_IDENTITY_OBJECT_HIGHEST_SUB_INDEX_SUPPORTED: Variable =
    Variable::new(0x1018, 0x00, Some(UNSIGNED8(0x4)));
pub const OD_IDENTITY_OBJECT_VENDOR_ID: Variable =
    Variable::new(0x1018, 0x01, None);
pub const OD_IDENTITY_OBJECT_PRODUCT_CODE: Variable =
    Variable::new(0x1018, 0x02, None);
pub const OD_IDENTITY_OBJECT_REVISION_NUMBER: Variable =
    Variable::new(0x1018, 0x03, None);
pub const OD_IDENTITY_OBJECT_SERIAL_NUMBER: Variable =
    Variable::new(0x1018, 0x04, None);
pub const OD_RECEIVE_PDO_0_COMMUNICATION_PARAMETER_NUMBER_OF_ENTRIES: Variable =
    Variable::new(0x1400, 0x00, Some(UNSIGNED8(0x2)));
pub const OD_RECEIVE_PDO_0_COMMUNICATION_PARAMETER_COB_ID_USE_BY_RPDO_1: Variable =
    Variable::new(0x1400, 0x01, Some(UNSIGNED32(0x202)));
pub const OD_RECEIVE_PDO_0_COMMUNICATION_PARAMETER_TRANSMISSION_TYPE_RPDO_1: Variable =
    Variable::new(0x1400, 0x02, Some(UNSIGNED8(0xff)));
pub const OD_RECEIVE_PDO_1_COMMUNICATION_PARAMETER_NUMBER_OF_ENTRIES: Variable =
    Variable::new(0x1401, 0x00, Some(UNSIGNED8(0x2)));
pub const OD_RECEIVE_PDO_1_COMMUNICATION_PARAMETER_COB_ID_USE_BY_RPDO_2: Variable =
    Variable::new(0x1401, 0x01, Some(UNSIGNED32(0x302)));
pub const OD_RECEIVE_PDO_1_COMMUNICATION_PARAMETER_TRANSMISSION_TYPE_RPDO_2: Variable =
    Variable::new(0x1401, 0x02, Some(UNSIGNED8(0xff)));
pub const OD_RECEIVE_PDO_2_COMMUNICATION_PARAMETER_NUMBER_OF_ENTRIES: Variable =
    Variable::new(0x1402, 0x00, Some(UNSIGNED8(0x2)));
pub const OD_RECEIVE_PDO_2_COMMUNICATION_PARAMETER_COB_ID_USE_BY_RPDO_3: Variable =
    Variable::new(0x1402, 0x01, Some(UNSIGNED32(0x402)));
pub const OD_RECEIVE_PDO_2_COMMUNICATION_PARAMETER_TRANSMISSION_TYPE_RPDO_3: Variable =
    Variable::new(0x1402, 0x02, Some(UNSIGNED8(0xff)));
pub const OD_RECEIVE_PDO_3_COMMUNICATION_PARAMETER_NUMBER_OF_ENTRIES: Variable =
    Variable::new(0x1403, 0x00, Some(UNSIGNED8(0x2)));
pub const OD_RECEIVE_PDO_3_COMMUNICATION_PARAMETER_COB_ID_USE_BY_RPDO_4: Variable =
    Variable::new(0x1403, 0x01, Some(UNSIGNED32(0x502)));
pub const OD_RECEIVE_PDO_3_COMMUNICATION_PARAMETER_TRANSMISSION_TYPE_RPDO_4: Variable =
    Variable::new(0x1403, 0x02, Some(UNSIGNED8(0xff)));
pub const OD_RECEIVE_PDO_0_MAPPING_PARAMETER_NUMBER_OF_MAPPED_OBJECTS_RPDO_1: Variable =
    Variable::new(0x1600, 0x00, Some(UNSIGNED8(0x1)));
pub const OD_RECEIVE_PDO_0_MAPPING_PARAMETER_RPDO_1_MAPPING_INFORMATION_1: Variable =
    Variable::new(0x1600, 0x01, Some(UNSIGNED32(0x60400010)));
pub const OD_RECEIVE_PDO_0_MAPPING_PARAMETER_RPDO_1_MAPPING_INFORMATION_2: Variable =
    Variable::new(0x1600, 0x02, Some(UNSIGNED32(0x0)));
pub const OD_RECEIVE_PDO_0_MAPPING_PARAMETER_RPDO_1_MAPPING_INFORMATION_3: Variable =
    Variable::new(0x1600, 0x03, Some(UNSIGNED32(0x0)));
pub const OD_RECEIVE_PDO_0_MAPPING_PARAMETER_RPDO_1_MAPPING_INFORMATION_4: Variable =
    Variable::new(0x1600, 0x04, Some(UNSIGNED32(0x0)));
pub const OD_RECEIVE_PDO_1_MAPPING_PARAMETER_NUMBER_OF_MAPPED_OBJECTS_RPDO_2: Variable =
    Variable::new(0x1601, 0x00, Some(UNSIGNED8(0x2)));
pub const OD_RECEIVE_PDO_1_MAPPING_PARAMETER_RPDO_2_MAPPING_INFORMATION_1: Variable =
    Variable::new(0x1601, 0x01, Some(UNSIGNED32(0x60400010)));
pub const OD_RECEIVE_PDO_1_MAPPING_PARAMETER_RPDO_2_MAPPING_INFORMATION_2: Variable =
    Variable::new(0x1601, 0x02, Some(UNSIGNED32(0x60600008)));
pub const OD_RECEIVE_PDO_1_MAPPING_PARAMETER_RPDO_2_MAPPING_INFORMATION_3: Variable =
    Variable::new(0x1601, 0x03, Some(UNSIGNED32(0x0)));
pub const OD_RECEIVE_PDO_1_MAPPING_PARAMETER_RPDO_2_MAPPING_INFORMATION_4: Variable =
    Variable::new(0x1601, 0x04, Some(UNSIGNED32(0x0)));
pub const OD_RECEIVE_PDO_2_MAPPING_PARAMETER_NUMBER_OF_MAPPED_OBJECTS_RPDO_3: Variable =
    Variable::new(0x1602, 0x00, Some(UNSIGNED8(0x2)));
pub const OD_RECEIVE_PDO_2_MAPPING_PARAMETER_RPDO_3_MAPPING_INFORMATION_1: Variable =
    Variable::new(0x1602, 0x01, Some(UNSIGNED32(0x60400010)));
pub const OD_RECEIVE_PDO_2_MAPPING_PARAMETER_RPDO_3_MAPPING_INFORMATION_2: Variable =
    Variable::new(0x1602, 0x02, Some(UNSIGNED32(0x607a0020)));
pub const OD_RECEIVE_PDO_2_MAPPING_PARAMETER_RPDO_3_MAPPING_INFORMATION_3: Variable =
    Variable::new(0x1602, 0x03, Some(UNSIGNED32(0x0)));
pub const OD_RECEIVE_PDO_2_MAPPING_PARAMETER_RPDO_3_MAPPING_INFORMATION_4: Variable =
    Variable::new(0x1602, 0x04, Some(UNSIGNED32(0x0)));
pub const OD_RECEIVE_PDO_3_MAPPING_PARAMETER_NUMBER_OF_MAPPED_OBJECTS_RPDO_4: Variable =
    Variable::new(0x1603, 0x00, Some(UNSIGNED8(0x2)));
pub const OD_RECEIVE_PDO_3_MAPPING_PARAMETER_RPDO_4_MAPPING_INFORMATION_1: Variable =
    Variable::new(0x1603, 0x01, Some(UNSIGNED32(0x60400010)));
pub const OD_RECEIVE_PDO_3_MAPPING_PARAMETER_RPDO_4_MAPPING_INFORMATION_2: Variable =
    Variable::new(0x1603, 0x02, Some(UNSIGNED32(0x60ff0020)));
pub const OD_RECEIVE_PDO_3_MAPPING_PARAMETER_RPDO_4_MAPPING_INFORMATION_3: Variable =
    Variable::new(0x1603, 0x03, Some(UNSIGNED32(0x0)));
pub const OD_RECEIVE_PDO_3_MAPPING_PARAMETER_RPDO_4_MAPPING_INFORMATION_4: Variable =
    Variable::new(0x1603, 0x04, Some(UNSIGNED32(0x0)));
pub const OD_TRANSMIT_PDO_0_COMMUNICATION_PARAMETERS_NUMBER_OF_ENTRIES: Variable =
    Variable::new(0x1800, 0x00, Some(UNSIGNED8(0x2)));
pub const OD_TRANSMIT_PDO_0_COMMUNICATION_PARAMETERS_COB_ID_USE_BY_TPDO_1: Variable =
    Variable::new(0x1800, 0x01, Some(UNSIGNED32(0x182)));
pub const OD_TRANSMIT_PDO_0_COMMUNICATION_PARAMETERS_TRANSMISSION_TYPE_TPDO_1: Variable =
    Variable::new(0x1800, 0x02, Some(UNSIGNED8(0xff)));
pub const OD_TRANSMIT_PDO_1_COMMUNICATION_PARAMETERS_NUMBER_OF_ENTRIES: Variable =
    Variable::new(0x1801, 0x00, Some(UNSIGNED8(0x2)));
pub const OD_TRANSMIT_PDO_1_COMMUNICATION_PARAMETERS_COB_ID_USE_BY_TPDO_2: Variable =
    Variable::new(0x1801, 0x01, Some(UNSIGNED32(0x282)));
pub const OD_TRANSMIT_PDO_1_COMMUNICATION_PARAMETERS_TRANSMISSION_TYPE_TPDO_2: Variable =
    Variable::new(0x1801, 0x02, Some(UNSIGNED8(0xff)));
pub const OD_TRANSMIT_PDO_2_COMMUNICATION_PARAMETERS_NUMBER_OF_ENTRIES: Variable =
    Variable::new(0x1802, 0x00, Some(UNSIGNED8(0x2)));
pub const OD_TRANSMIT_PDO_2_COMMUNICATION_PARAMETERS_COB_ID_USE_BY_TPDO_3: Variable =
    Variable::new(0x1802, 0x01, Some(UNSIGNED32(0x382)));
pub const OD_TRANSMIT_PDO_2_COMMUNICATION_PARAMETERS_TRANSMISSION_TYPE_TPDO_3: Variable =
    Variable::new(0x1802, 0x02, Some(UNSIGNED8(0x0)));
pub const OD_TRANSMIT_PDO_3_COMMUNICATION_PARAMETERS_NUMBER_OF_ENTRIES: Variable =
    Variable::new(0x1803, 0x00, Some(UNSIGNED8(0x2)));
pub const OD_TRANSMIT_PDO_3_COMMUNICATION_PARAMETERS_COB_ID_USE_BY_TPDO_4: Variable =
    Variable::new(0x1803, 0x01, Some(UNSIGNED32(0x482)));
pub const OD_TRANSMIT_PDO_3_COMMUNICATION_PARAMETERS_TRANSMISSION_TYPE_TPDO_4: Variable =
    Variable::new(0x1803, 0x02, Some(UNSIGNED8(0x0)));
pub const OD_TRANSMIT_PDO_0_MAPPING_PARAMETER_NUMBER_OF_MAPPED_OBJECTS_TPDO_1: Variable =
    Variable::new(0x1a00, 0x00, Some(UNSIGNED8(0x1)));
pub const OD_TRANSMIT_PDO_0_MAPPING_PARAMETER_TPDO_1_MAPPING_INFORMATION_1: Variable =
    Variable::new(0x1a00, 0x01, Some(UNSIGNED32(0x60410010)));
pub const OD_TRANSMIT_PDO_0_MAPPING_PARAMETER_TPDO_1_MAPPING_INFORMATION_2: Variable =
    Variable::new(0x1a00, 0x02, Some(UNSIGNED32(0x0)));
pub const OD_TRANSMIT_PDO_0_MAPPING_PARAMETER_TPDO_1_MAPPING_INFORMATION_3: Variable =
    Variable::new(0x1a00, 0x03, Some(UNSIGNED32(0x0)));
pub const OD_TRANSMIT_PDO_0_MAPPING_PARAMETER_TPDO_1_MAPPING_INFORMATION_4: Variable =
    Variable::new(0x1a00, 0x04, Some(UNSIGNED32(0x0)));
pub const OD_TRANSMIT_PDO_1_MAPPING_PARAMETER_NUMBER_OF_MAPPED_OBJECTS_TPDO_2: Variable =
    Variable::new(0x1a01, 0x00, Some(UNSIGNED8(0x2)));
pub const OD_TRANSMIT_PDO_1_MAPPING_PARAMETER_TPDO_2_MAPPING_INFORMATION_1: Variable =
    Variable::new(0x1a01, 0x01, Some(UNSIGNED32(0x60410010)));
pub const OD_TRANSMIT_PDO_1_MAPPING_PARAMETER_TPDO_2_MAPPING_INFORMATION_2: Variable =
    Variable::new(0x1a01, 0x02, Some(UNSIGNED32(0x60610008)));
pub const OD_TRANSMIT_PDO_1_MAPPING_PARAMETER_TPDO_2_MAPPING_INFORMATION_3: Variable =
    Variable::new(0x1a01, 0x03, Some(UNSIGNED32(0x0)));
pub const OD_TRANSMIT_PDO_1_MAPPING_PARAMETER_TPDO_2_MAPPING_INFORMATION_4: Variable =
    Variable::new(0x1a01, 0x04, Some(UNSIGNED32(0x0)));
pub const OD_TRANSMIT_PDO_2_MAPPING_PARAMETER_NUMBER_OF_MAPPED_OBJECTS_TPDO_3: Variable =
    Variable::new(0x1a02, 0x00, Some(UNSIGNED8(0x2)));
pub const OD_TRANSMIT_PDO_2_MAPPING_PARAMETER_TPDO_3_MAPPING_INFORMATION_1: Variable =
    Variable::new(0x1a02, 0x01, Some(UNSIGNED32(0x60410010)));
pub const OD_TRANSMIT_PDO_2_MAPPING_PARAMETER_TPDO_3_MAPPING_INFORMATION_2: Variable =
    Variable::new(0x1a02, 0x02, Some(UNSIGNED32(0x60640020)));
pub const OD_TRANSMIT_PDO_2_MAPPING_PARAMETER_TPDO_3_MAPPING_INFORMATION_3: Variable =
    Variable::new(0x1a02, 0x03, Some(UNSIGNED32(0x0)));
pub const OD_TRANSMIT_PDO_2_MAPPING_PARAMETER_TPDO_3_MAPPING_INFORMATION_4: Variable =
    Variable::new(0x1a02, 0x04, Some(UNSIGNED32(0x0)));
pub const OD_TRANSMIT_PDO_3_MAPPING_PARAMETER_NUMBER_OF_MAPPED_OBJECTS_TPDO_4: Variable =
    Variable::new(0x1a03, 0x00, Some(UNSIGNED8(0x2)));
pub const OD_TRANSMIT_PDO_3_MAPPING_PARAMETER_TPDO_4_MAPPING_INFORMATION_1: Variable =
    Variable::new(0x1a03, 0x01, Some(UNSIGNED32(0x60410010)));
pub const OD_TRANSMIT_PDO_3_MAPPING_PARAMETER_TPDO_4_MAPPING_INFORMATION_2: Variable =
    Variable::new(0x1a03, 0x02, Some(UNSIGNED32(0x606c0020)));
pub const OD_TRANSMIT_PDO_3_MAPPING_PARAMETER_TPDO_4_MAPPING_INFORMATION_3: Variable =
    Variable::new(0x1a03, 0x03, Some(UNSIGNED32(0x0)));
pub const OD_TRANSMIT_PDO_3_MAPPING_PARAMETER_TPDO_4_MAPPING_INFORMATION_4: Variable =
    Variable::new(0x1a03, 0x04, Some(UNSIGNED32(0x0)));
pub const OD_WRITABLE_STRING: Variable =
    Variable::new(0x2000, 0x00, None);
pub const OD_INTEGER16_VALUE: Variable =
    Variable::new(0x2001, 0x00, None);
pub const OD_UNSIGNED8_VALUE: Variable =
    Variable::new(0x2002, 0x00, None);
pub const OD_INTEGER8_VALUE: Variable =
    Variable::new(0x2003, 0x00, None);
pub const OD_INTEGER32_VALUE: Variable =
    Variable::new(0x2004, 0x00, None);
pub const OD_BOOLEAN_VALUE: Variable =
    Variable::new(0x2005, 0x00, None);
pub const OD_BOOLEAN_VALUE_2: Variable =
    Variable::new(0x2006, 0x00, None);
pub const OD_SENSOR_SAMPLING_RATE_HZ: Variable =
    Variable::new(0x3002, 0x00, Some(REAL32(5.2)));
pub const OD_SENSOR_STATUS_NUMBER_OF_ENTRIES: Variable =
    Variable::new(0x3004, 0x00, None);
pub const OD_SENSOR_STATUS_SENSOR_STATUS_1: Variable =
    Variable::new(0x3004, 0x01, Some(UNSIGNED16(0x3)));
pub const OD_SENSOR_STATUS_SENSOR_STATUS_2: Variable =
    Variable::new(0x3004, 0x02, Some(UNSIGNED16(0x3)));
pub const OD_SENSOR_STATUS_SENSOR_STATUS_3: Variable =
    Variable::new(0x3004, 0x03, Some(UNSIGNED16(0x3)));
pub const OD_VALVE_1__OPEN_NUMBER_OF_ENTRIES: Variable =
    Variable::new(0x3006, 0x00, None);
pub const OD_VALVE_1__OPEN_VALVE_1__OPEN: Variable =
    Variable::new(0x3006, 0x01, None);
pub const OD_READRAWVALUE_TEMPERATURE: Variable =
    Variable::new(0x3010, 0x00, Some(REAL32(0.0)));

pub static OD: [Variable; 92] = [
    OD_DUMMY0001,
    OD_DUMMY0002,
    OD_DEVICE_TYPE,
    OD_ERROR_REGISTER,
    OD_PRE_DEFINED_ERROR_FIELD_NUMBER_OF_ENTRIES,
    OD_PRE_DEFINED_ERROR_FIELD_PRE_DEFINED_ERROR_FIELD,
    OD_MANUFACTURER_DEVICE_NAME,
    OD_PRODUCER_HEARTBEAT_TIME,
    OD_IDENTITY_OBJECT_HIGHEST_SUB_INDEX_SUPPORTED,
    OD_IDENTITY_OBJECT_VENDOR_ID,
    OD_IDENTITY_OBJECT_PRODUCT_CODE,
    OD_IDENTITY_OBJECT_REVISION_NUMBER,
    OD_IDENTITY_OBJECT_SERIAL_NUMBER,
    OD_RECEIVE_PDO_0_COMMUNICATION_PARAMETER_NUMBER_OF_ENTRIES,
    OD_RECEIVE_PDO_0_COMMUNICATION_PARAMETER_COB_ID_USE_BY_RPDO_1,
    OD_RECEIVE_PDO_0_COMMUNICATION_PARAMETER_TRANSMISSION_TYPE_RPDO_1,
    OD_RECEIVE_PDO_1_COMMUNICATION_PARAMETER_NUMBER_OF_ENTRIES,
    OD_RECEIVE_PDO_1_COMMUNICATION_PARAMETER_COB_ID_USE_BY_RPDO_2,
    OD_RECEIVE_PDO_1_COMMUNICATION_PARAMETER_TRANSMISSION_TYPE_RPDO_2,
    OD_RECEIVE_PDO_2_COMMUNICATION_PARAMETER_NUMBER_OF_ENTRIES,
    OD_RECEIVE_PDO_2_COMMUNICATION_PARAMETER_COB_ID_USE_BY_RPDO_3,
    OD_RECEIVE_PDO_2_COMMUNICATION_PARAMETER_TRANSMISSION_TYPE_RPDO_3,
    OD_RECEIVE_PDO_3_COMMUNICATION_PARAMETER_NUMBER_OF_ENTRIES,
    OD_RECEIVE_PDO_3_COMMUNICATION_PARAMETER_COB_ID_USE_BY_RPDO_4,
    OD_RECEIVE_PDO_3_COMMUNICATION_PARAMETER_TRANSMISSION_TYPE_RPDO_4,
    OD_RECEIVE_PDO_0_MAPPING_PARAMETER_NUMBER_OF_MAPPED_OBJECTS_RPDO_1,
    OD_RECEIVE_PDO_0_MAPPING_PARAMETER_RPDO_1_MAPPING_INFORMATION_1,
    OD_RECEIVE_PDO_0_MAPPING_PARAMETER_RPDO_1_MAPPING_INFORMATION_2,
    OD_RECEIVE_PDO_0_MAPPING_PARAMETER_RPDO_1_MAPPING_INFORMATION_3,
    OD_RECEIVE_PDO_0_MAPPING_PARAMETER_RPDO_1_MAPPING_INFORMATION_4,
    OD_RECEIVE_PDO_1_MAPPING_PARAMETER_NUMBER_OF_MAPPED_OBJECTS_RPDO_2,
    OD_RECEIVE_PDO_1_MAPPING_PARAMETER_RPDO_2_MAPPING_INFORMATION_1,
    OD_RECEIVE_PDO_1_MAPPING_PARAMETER_RPDO_2_MAPPING_INFORMATION_2,
    OD_RECEIVE_PDO_1_MAPPING_PARAMETER_RPDO_2_MAPPING_INFORMATION_3,
    OD_RECEIVE_PDO_1_MAPPING_PARAMETER_RPDO_2_MAPPING_INFORMATION_4,
    OD_RECEIVE_PDO_2_MAPPING_PARAMETER_NUMBER_OF_MAPPED_OBJECTS_RPDO_3,
    OD_RECEIVE_PDO_2_MAPPING_PARAMETER_RPDO_3_MAPPING_INFORMATION_1,
    OD_RECEIVE_PDO_2_MAPPING_PARAMETER_RPDO_3_MAPPING_INFORMATION_2,
    OD_RECEIVE_PDO_2_MAPPING_PARAMETER_RPDO_3_MAPPING_INFORMATION_3,
    OD_RECEIVE_PDO_2_MAPPING_PARAMETER_RPDO_3_MAPPING_INFORMATION_4,
    OD_RECEIVE_PDO_3_MAPPING_PARAMETER_NUMBER_OF_MAPPED_OBJECTS_RPDO_4,
    OD_RECEIVE_PDO_3_MAPPING_PARAMETER_RPDO_4_MAPPING_INFORMATION_1,
    OD_RECEIVE_PDO_3_MAPPING_PARAMETER_RPDO_4_MAPPING_INFORMATION_2,
    OD_RECEIVE_PDO_3_MAPPING_PARAMETER_RPDO_4_MAPPING_INFORMATION_3,
    OD_RECEIVE_PDO_3_MAPPING_PARAMETER_RPDO_4_MAPPING_INFORMATION_4,
    OD_TRANSMIT_PDO_0_COMMUNICATION_PARAMETERS_NUMBER_OF_ENTRIES,
    OD_TRANSMIT_PDO_0_COMMUNICATION_PARAMETERS_COB_ID_USE_BY_TPDO_1,
    OD_TRANSMIT_PDO_0_COMMUNICATION_PARAMETERS_TRANSMISSION_TYPE_TPDO_1,
    OD_TRANSMIT_PDO_1_COMMUNICATION_PARAMETERS_NUMBER_OF_ENTRIES,
    OD_TRANSMIT_PDO_1_COMMUNICATION_PARAMETERS_COB_ID_USE_BY_TPDO_2,
    OD_TRANSMIT_PDO_1_COMMUNICATION_PARAMETERS_TRANSMISSION_TYPE_TPDO_2,
    OD_TRANSMIT_PDO_2_COMMUNICATION_PARAMETERS_NUMBER_OF_ENTRIES,
    OD_TRANSMIT_PDO_2_COMMUNICATION_PARAMETERS_COB_ID_USE_BY_TPDO_3,
    OD_TRANSMIT_PDO_2_COMMUNICATION_PARAMETERS_TRANSMISSION_TYPE_TPDO_3,
    OD_TRANSMIT_PDO_3_COMMUNICATION_PARAMETERS_NUMBER_OF_ENTRIES,
    OD_TRANSMIT_PDO_3_COMMUNICATION_PARAMETERS_COB_ID_USE_BY_TPDO_4,
    OD_TRANSMIT_PDO_3_COMMUNICATION_PARAMETERS_TRANSMISSION_TYPE_TPDO_4,
    OD_TRANSMIT_PDO_0_MAPPING_PARAMETER_NUMBER_OF_MAPPED_OBJECTS_TPDO_1,
    OD_TRANSMIT_PDO_0_MAPPING_PARAMETER_TPDO_1_MAPPING_INFORMATION_1,
    OD_TRANSMIT_PDO_0_MAPPING_PARAMETER_TPDO_1_MAPPING_INFORMATION_2,
    OD_TRANSMIT_PDO_0_MAPPING_PARAMETER_TPDO_1_MAPPING_INFORMATION_3,
    OD_TRANSMIT_PDO_0_MAPPING_PARAMETER_TPDO_1_MAPPING_INFORMATION_4,
    OD_TRANSMIT_PDO_1_MAPPING_PARAMETER_NUMBER_OF_MAPPED_OBJECTS_TPDO_2,
    OD_TRANSMIT_PDO_1_MAPPING_PARAMETER_TPDO_2_MAPPING_INFORMATION_1,
    OD_TRANSMIT_PDO_1_MAPPING_PARAMETER_TPDO_2_MAPPING_INFORMATION_2,
    OD_TRANSMIT_PDO_1_MAPPING_PARAMETER_TPDO_2_MAPPING_INFORMATION_3,
    OD_TRANSMIT_PDO_1_MAPPING_PARAMETER_TPDO_2_MAPPING_INFORMATION_4,
    OD_TRANSMIT_PDO_2_MAPPING_PARAMETER_NUMBER_OF_MAPPED_OBJECTS_TPDO_3,
    OD_TRANSMIT_PDO_2_MAPPING_PARAMETER_TPDO_3_MAPPING_INFORMATION_1,
    OD_TRANSMIT_PDO_2_MAPPING_PARAMETER_TPDO_3_MAPPING_INFORMATION_2,
    OD_TRANSMIT_PDO_2_MAPPING_PARAMETER_TPDO_3_MAPPING_INFORMATION_3,
    OD_TRANSMIT_PDO_2_MAPPING_PARAMETER_TPDO_3_MAPPING_INFORMATION_4,
    OD_TRANSMIT_PDO_3_MAPPING_PARAMETER_NUMBER_OF_MAPPED_OBJECTS_TPDO_4,
    OD_TRANSMIT_PDO_3_MAPPING_PARAMETER_TPDO_4_MAPPING_INFORMATION_1,
    OD_TRANSMIT_PDO_3_MAPPING_PARAMETER_TPDO_4_MAPPING_INFORMATION_2,
    OD_TRANSMIT_PDO_3_MAPPING_PARAMETER_TPDO_4_MAPPING_INFORMATION_3,
    OD_TRANSMIT_PDO_3_MAPPING_PARAMETER_TPDO_4_MAPPING_INFORMATION_4,
    OD_WRITABLE_STRING,
    OD_INTEGER16_VALUE,
    OD_UNSIGNED8_VALUE,
    OD_INTEGER8_VALUE,
    OD_INTEGER32_VALUE,
    OD_BOOLEAN_VALUE,
    OD_BOOLEAN_VALUE_2,
    OD_SENSOR_SAMPLING_RATE_HZ,
    OD_SENSOR_STATUS_NUMBER_OF_ENTRIES,
    OD_SENSOR_STATUS_SENSOR_STATUS_1,
    OD_SENSOR_STATUS_SENSOR_STATUS_2,
    OD_SENSOR_STATUS_SENSOR_STATUS_3,
    OD_VALVE_1__OPEN_NUMBER_OF_ENTRIES,
    OD_VALVE_1__OPEN_VALVE_1__OPEN,
    OD_READRAWVALUE_TEMPERATURE,
];
