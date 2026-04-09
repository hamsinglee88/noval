const usernamePattern = /^[A-Za-z0-9_]{3,20}$/;

export function validateUsername(value: string): string {
  if (!value) {
    return '请输入用户名。';
  }

  if (!usernamePattern.test(value)) {
    return '用户名需为 3-20 位，仅允许字母、数字和下划线。';
  }

  return '';
}

export interface PasswordStrength {
  level: 0 | 1 | 2 | 3;
  label: string;
  message: string;
}

export function getPasswordStrength(value: string): PasswordStrength {
  if (!value) {
    return {
      level: 0,
      label: '未输入',
      message: '密码至少 8 位，且必须同时包含字母和数字。',
    };
  }

  const hasLetter = /[A-Za-z]/.test(value);
  const hasNumber = /\d/.test(value);

  if (value.length < 8 || !hasLetter || !hasNumber) {
    return {
      level: 1,
      label: '弱',
      message: '密码至少 8 位，且必须同时包含字母和数字。',
    };
  }

  if (value.length >= 12 && /[^A-Za-z0-9]/.test(value)) {
    return {
      level: 3,
      label: '强',
      message: '密码强度较高，可以使用。',
    };
  }

  return {
    level: 2,
    label: '中',
    message: '密码合格，建议补充特殊字符提升强度。',
  };
}
