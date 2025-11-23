use jieto_web::BusinessError;

pub const _INVALID_ID: BusinessError = BusinessError {
    code: 4001,
    msg: "无效的ID",
};

pub const _USER_NOT_FOUND: BusinessError = BusinessError {
    code: 4002,
    msg: "用户不存在",
};

pub const _PERMISSION_DENIED: BusinessError = BusinessError {
    code: 4003,
    msg: "权限不足",
};