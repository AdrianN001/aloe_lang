use crate::test::util::test_cases_for_input_output;

#[test]
pub fn test_math_module() {
    let testcases = [
        ("import {sqrt} from \"@std::math\"; sqrt(4);", "2"),
        ("import {ceil} from \"@std::math\"; ceil(2.3);", "3"),
        ("import {floor} from \"@std::math\"; floor(-2.3);", "-3"),
        ("import {trunc} from \"@std::math\"; trunc(-2.9);", "-2"),
        ("import {fabs} from \"@std::math\"; fabs(-3);", "3"),
        (
            "import {exp} from \"@std::math\"; exp(1);",
            "2.718281828459045",
        ),
        (
            "import {expm1} from \"@std::math\"; expm1(1);",
            "1.718281828459045",
        ),
        ("import {log} from \"@std::math\"; log(8, 2);", "3"),
        ("import {log10} from \"@std::math\"; log10(100);", "2"),
        (
            "import {log1p} from \"@std::math\"; log1p(1);",
            "0.6931471805599453",
        ),
        ("import {pow} from \"@std::math\"; pow(2, 3);", "8"),
        ("import {fmod} from \"@std::math\"; fmod(7, 3);", "1"),
        ("import {hypot} from \"@std::math\"; hypot(3, 4);", "5"),
        (
            "import {copysign} from \"@std::math\"; copysign(-3, 2);",
            "3",
        ),
        (
            "import {degrees} from \"@std::math\"; degrees(3.141592653589793);",
            "180",
        ),
        (
            "import {radians} from \"@std::math\"; radians(180);",
            "3.141592653589793",
        ),
        (
            "import {sin} from \"@std::math\"; sin(1.5707963267948966);",
            "1",
        ),
        (
            "import {cos} from \"@std::math\"; cos(3.141592653589793);",
            "-1",
        ),
        ("import {tan} from \"@std::math\"; tan(0);", "0"),
        (
            "import {asin} from \"@std::math\"; asin(1);",
            "1.5707963267948966",
        ),
        ("import {acos} from \"@std::math\"; acos(1);", "0"),
        (
            "import {atan} from \"@std::math\"; atan(1);",
            "0.7853981633974483",
        ),
        (
            "import {atan2} from \"@std::math\"; atan2(0, -1);",
            "3.141592653589793",
        ),
        ("import {sinh} from \"@std::math\"; sinh(0);", "0"),
        ("import {cosh} from \"@std::math\"; cosh(0);", "1"),
        ("import {tanh} from \"@std::math\"; tanh(0);", "0"),
        ("import {asinh} from \"@std::math\"; asinh(0);", "0"),
        ("import {acosh} from \"@std::math\"; acosh(1);", "0"),
        ("import {atanh} from \"@std::math\"; atanh(0);", "0"),
        ("import {ldexp} from \"@std::math\"; ldexp(0.5, 4);", "8"),
        ("import {frexp} from \"@std::math\"; frexp(8);", "[0.5, 4]"),
        ("import {modf} from \"@std::math\"; modf(2.5);", "[0.5, 2]"),
        ("import {gcd} from \"@std::math\"; gcd(12, 18);", "6"),
        ("import {lcm} from \"@std::math\"; lcm(4, 6);", "12"),
        (
            "import {factorial} from \"@std::math\"; factorial(5);",
            "120",
        ),
    ];

    test_cases_for_input_output(&testcases);
}
