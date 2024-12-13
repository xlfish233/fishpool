use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct ReqCreate {
    #[validate(length(min = 1, max = 255, message = "用户名必填且长度不能超过255个字符"))]
    pub username: String,
    #[validate(length(min = 1, max = 255, message = "密码必填且长度不能超过255个字符"))]
    pub password: String,
}

#[cfg(test)]
mod tests {
    use super::ReqCreate;
    use validator::Validate;

    // 测试用例：验证ReqCreate实例在用户名和密码都有效时是否通过验证
    #[test]
    fn test_req_create_valid() {
        let user = ReqCreate {
            username: "user".to_string(),
            password: "pass".to_string(),
        };

        assert!(user.validate().is_ok());
    }

    // 测试用例：验证ReqCreate实例在用户名为空时是否失败验证
    #[test]
    fn test_req_create_invalid_username() {
        let user = ReqCreate {
            username: "".to_string(),
            password: "pass".to_string(),
        };
        assert!(user.validate().is_err());
    }

    // 测试用例：验证ReqCreate实例在密码为空时是否失败验证
    #[test]
    fn test_req_create_invalid_password() {
        let user = ReqCreate {
            username: "user".to_string(),
            password: "".to_string(),
        };
        assert!(user.validate().is_err());
    }

    // 测试用例：验证ReqCreate实例在用户名过长时是否失败验证
    #[test]
    fn test_req_create_invalid_username_too_long() {
        let user = ReqCreate {
            username: "a".repeat(256),
            password: "pass".to_string(),
        };
        assert!(user.validate().is_err());
    }

    // 测试用例：验证ReqCreate实例在密码过长时是否失败验证
    #[test]
    fn test_req_create_invalid_password_too_long() {
        let user = ReqCreate {
            username: "user".to_string(),
            password: "a".repeat(256),
        };
        assert!(user.validate().is_err());
    }
}
