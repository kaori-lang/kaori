def test_simple_counter():
    result = 0
    i = 0
    while i < 100:
        result += i
        i += 1
    return result


def test_nested_loops_sum():
    result = 0
    i = 0
    while i < 20:
        j = 0
        while j < 20:
            result += i * j
            j += 1
        i += 1
    return result


def test_triple_nested_loops():
    result = 0
    i = 1
    while i < 10:
        j = 1
        while j < 10:
            k = 1
            while k < 10:
                result += i + j + k
                k += 1
            j += 1
        i += 1
    return result


def test_if_else_basic():
    x = 10
    y = 7
    result = 0
    if x > y:
        result = x * 2
    else:
        result = y * 2
    return result


def test_nested_if_else():
    x = 10
    y = 7
    z = 3
    result = 0
    if x > 8:
        if y > 5:
            if z > 4:
                result = x * y * z
            else:
                result = x * y + z
        else:
            result = x + y
    else:
        result = z
    return result


def test_if_else_with_and_or():
    x = 10
    y = 7
    z = 3
    w = 5
    result = 0
    if x > 8 and y > 5:
        if z > 4 or w > 4:
            result = x * y * w
        else:
            result = x + y + z + w
    else:
        if x > 5 or y > 6:
            result = x * w + y
        else:
            result = z * w
    return result


def test_loop_with_if_else():
    result = 0
    i = 0
    while i < 20:
        if i * i > 100:
            result += i * 2
        else:
            result += i
        i += 1
    return result


def test_nested_loop_with_and_condition():
    result = 0
    i = 0
    while i < 15:
        j = 0
        while j < 15:
            if i > 5 and j > 5:
                result += i * j
            else:
                result += i + j
            j += 1
        i += 1
    return result


def test_break_inner_loop():
    result = 0
    i = 0
    while i < 10:
        j = 0
        while j < 10:
            if j == 5:
                break
            result += j
            j += 1
        i += 1
    return result


def test_continue_inner_loop():
    result = 0
    i = 0
    while i < 10:
        j = 0
        while j < 10:
            if j == 3:
                j += 1
                continue
            result += j
            j += 1
        i += 1
    return result


def test_break_outer_loop():
    result = 0
    i = 0
    while i < 10:
        j = 0
        while j < 10:
            result += 1
            j += 1
        if i == 5:
            break
        i += 1
    return result


def test_complex_break_continue():
    result = 0
    i = 0
    while i < 10:
        j = 0
        while j < 10:
            k = 0
            while k < 10:
                if k == 3:
                    k += 1
                    continue
                if j == 7 and k == 8:
                    break
                if i == 5 and j == 5 and k == 5:
                    break
                result += 1
                k += 1
            if i == 5 and j == 5:
                break
            j += 1
        i += 1
    return result


def test_arithmetic_expressions():
    x = 10
    y = 7
    z = 3
    w = 5
    v = 12
    result = 0
    if x * 2 > y + w and v - z > x:
        if x * y > v * w or z + w > x:
            result = x * y * z + v * w
        else:
            result = x * v + y * z - w
    else:
        if v * 2 > x * y or z * w > x + y:
            result = x * w * z + y - v
        else:
            result = x * y + v * z - w
    return result


def test_shadow_variable_in_loop():
    result = 0
    i = 8
    j = 7
    while i < 10:
        j = 0
        while j < 10:
            if i == 5 and j == 5:
                j += 1
            else:
                result += 1
                j += 1
        i += 1
    return result


def test_nested_and_conditions():
    result = 0
    i = 0
    while i < 20:
        j = 0
        while j < 20:
            if i > 5 and j > 5 and i + j > 15:
                result += i * j
            else:
                result += 1
            j += 1
        i += 1
    return result


def test_nested_or_conditions():
    result = 0
    i = 0
    while i < 20:
        j = 0
        while j < 20:
            if i == 0 or j == 0 or i == 19 or j == 19:
                result += 1
            else:
                result += i + j
            j += 1
        i += 1
    return result


def test_mixed_and_or():
    result = 0
    i = 0
    while i < 15:
        j = 0
        while j < 15:
            if i > 3 and j > 3 or i > 10 or j > 10:
                result += i * j
            else:
                result += i + j
            j += 1
        i += 1
    return result


def test_subtraction_accumulator():
    result = 1000
    i = 0
    while i < 50:
        j = 0
        while j < 10:
            if i > j:
                result -= 1
            else:
                result += j - i
            j += 1
        i += 1
    return result


def test_multiply_accumulator():
    result = 0
    i = 1
    while i < 8:
        j = 1
        while j < 8:
            k = 1
            while k < 8:
                if i * j * k < 50:
                    result += i * j * k
                k += 1
            j += 1
        i += 1
    return result


def test_division_condition():
    result = 0
    i = 1
    while i < 30:
        j = 1
        while j < 30:
            if i / j > 2 and i - j > 3:
                result += i * j
            else:
                result += i + j
            j += 1
        i += 1
    return result


def test_less_equal_greater_equal():
    result = 0
    i = 0
    while i < 20:
        j = 0
        while j < 20:
            if i >= 10 and j <= 10:
                result += i - j
            else:
                result += j - i
            j += 1
        i += 1
    return result


def test_not_equal_condition():
    result = 0
    i = 0
    while i < 20:
        j = 0
        while j < 20:
            if i != j and i != 0 and j != 0:
                result += i + j
            else:
                result += 1
            j += 1
        i += 1
    return result


def test_complex_arithmetic_in_condition():
    result = 0
    i = 1
    while i < 15:
        j = 1
        while j < 15:
            k = 1
            while k < 15:
                if i * j + k * 2 > i + j * k and i * k < j * j:
                    result += i + j + k
                else:
                    result += i * j - k
                k += 1
            j += 1
        i += 1
    return result


def test_deep_nested_if_else_with_and_or():
    x = 10
    y = 7
    z = 3
    w = 5
    v = 12
    result = 0
    if x * 2 > y + w and v - z > x:
        if x * y > v * w or z + w > x:
            if x > 9 and w > 3:
                result = x * y * w
            else:
                if z > 2 or y > 6:
                    result = x * y + z * w
                else:
                    result = x + y + z + w
        else:
            if y > 6 and w > 4:
                result = y * w + x
            else:
                result = x * z + y * w
    else:
        if x > 5 or y > 6:
            if y > 5 and z > 2:
                result = y * z * w
            else:
                result = x * 2 + y
        else:
            result = z * w * x
    return result


def test_break_on_complex_condition():
    result = 0
    i = 0
    while i < 20:
        j = 0
        while j < 20:
            if i * j > 100 and i + j > 15:
                break
            result += i + j
            j += 1
        i += 1
    return result


def test_continue_on_complex_condition():
    result = 0
    i = 0
    while i < 20:
        j = 0
        while j < 20:
            if i == j or i + j == 10:
                j += 1
                continue
            result += i * j
            j += 1
        i += 1
    return result


def test_multiple_breaks_different_depths():
    result = 0
    i = 0
    while i < 10:
        j = 0
        while j < 10:
            k = 0
            while k < 10:
                if k > 5:
                    break
                result += k
                k += 1
            if j > 5:
                break
            result += j
            j += 1
        if i > 5:
            break
        result += i
        i += 1
    return result


def test_alternating_add_subtract():
    result = 0
    i = 0
    while i < 30:
        j = 0
        while j < 30:
            if i > j:
                result += i * 2 - j
            else:
                result += j * 2 - i
            j += 1
        i += 1
    return result


def test_triple_and_condition_in_loop():
    result = 0
    i = 0
    while i < 20:
        j = 0
        while j < 20:
            k = 0
            while k < 20:
                if i > 5 and j > 5 and k > 5:
                    result += 1
                else:
                    result += i + j + k
                k += 1
            j += 1
        i += 1
    return result


def test_harmonic_sum_condition():
    result = 0
    i = 1
    while i < 30:
        j = 1
        while j < 30:
            if i / j > 1 and i * j < 200:
                result += i + j
            else:
                result += 1
            j += 1
        i += 1
    return result


def test_complex_multi_variable():
    a = 3
    b = 7
    c = 11
    d = 2
    result = 0
    i = 0
    while i < 25:
        if i * a > b * d and i + c < 30:
            result += i * a - b
        else:
            if i * b > c * d or i < a:
                result += i + b + c
            else:
                result += a * b - c * d
        i += 1
    return result


def test_nested_loop_continue_and_break():
    result = 0
    i = 0
    while i < 15:
        j = 0
        while j < 15:
            if j == 3 or j == 7:
                j += 1
                continue
            if i == 10 and j > 10:
                break
            result += i * j
            j += 1
        i += 1
    return result


def test_accumulate_with_not_equal_and_greater():
    result = 0
    i = 1
    while i < 25:
        j = 1
        while j < 25:
            if i != j and i > j and i * j > 50:
                result += i - j
            else:
                if i != j and j > i and i * j > 50:
                    result += j - i
                else:
                    result += 1
            j += 1
        i += 1
    return result


tests = [
    {
        "name": "simple counter",
        "fn": test_simple_counter,
        "kaori": """
let result = 0;
let i = 0;
while i < 100 {
    result = result + i;
    i = i + 1;
}
return result;
        """
    },
    {
        "name": "nested loops sum",
        "fn": test_nested_loops_sum,
        "kaori": """
let result = 0;
let i = 0;
while i < 20 {
    let j = 0;
    while j < 20 {
        result = result + i * j;
        j = j + 1;
    }
    i = i + 1;
}
return result;
        """
    },
    {
        "name": "triple nested loops",
        "fn": test_triple_nested_loops,
        "kaori": """
let result = 0;
let i = 1;
while i < 10 {
    let j = 1;
    while j < 10 {
        let k = 1;
        while k < 10 {
            result = result + i + j + k;
            k = k + 1;
        }
        j = j + 1;
    }
    i = i + 1;
}
return result;
        """
    },
    {
        "name": "if else basic",
        "fn": test_if_else_basic,
        "kaori": """
let x = 10;
let y = 7;
let result = 0;
if x > y {
    result = x * 2;
} else {
    result = y * 2;
}
return result;
        """
    },
    {
        "name": "nested if else",
        "fn": test_nested_if_else,
        "kaori": """
let x = 10;
let y = 7;
let z = 3;
let result = 0;
if x > 8 {
    if y > 5 {
        if z > 4 {
            result = x * y * z;
        } else {
            result = x * y + z;
        }
    } else {
        result = x + y;
    }
} else {
    result = z;
}
return result;
        """
    },
    {
        "name": "if else with and or",
        "fn": test_if_else_with_and_or,
        "kaori": """
let x = 10;
let y = 7;
let z = 3;
let w = 5;
let result = 0;
if x > 8 and y > 5 {
    if z > 4 or w > 4 {
        result = x * y * w;
    } else {
        result = x + y + z + w;
    }
} else {
    if x > 5 or y > 6 {
        result = x * w + y;
    } else {
        result = z * w;
    }
}
return result;
        """
    },
    {
        "name": "loop with if else",
        "fn": test_loop_with_if_else,
        "kaori": """
let result = 0;
let i = 0;
while i < 20 {
    if i * i > 100 {
        result = result + i * 2;
    } else {
        result = result + i;
    }
    i = i + 1;
}
return result;
        """
    },
    {
        "name": "nested loop with and condition",
        "fn": test_nested_loop_with_and_condition,
        "kaori": """
let result = 0;
let i = 0;
while i < 15 {
    let j = 0;
    while j < 15 {
        if i > 5 and j > 5 {
            result = result + i * j;
        } else {
            result = result + i + j;
        }
        j = j + 1;
    }
    i = i + 1;
}
return result;
        """
    },
    {
        "name": "break inner loop",
        "fn": test_break_inner_loop,
        "kaori": """
let result = 0;
let i = 0;
while i < 10 {
    let j = 0;
    while j < 10 {
        if j == 5 {
            break;
        }
        result = result + j;
        j = j + 1;
    }
    i = i + 1;
}
return result;
        """
    },
    {
        "name": "continue inner loop",
        "fn": test_continue_inner_loop,
        "kaori": """
let result = 0;
let i = 0;
while i < 10 {
    let j = 0;
    while j < 10 {
        if j == 3 {
            j = j + 1;
            continue;
        }
        result = result + j;
        j = j + 1;
    }
    i = i + 1;
}
return result;
        """
    },
    {
        "name": "break outer loop",
        "fn": test_break_outer_loop,
        "kaori": """
let result = 0;
let i = 0;
while i < 10 {
    let j = 0;
    while j < 10 {
        result = result + 1;
        j = j + 1;
    }
    if i == 5 {
        break;
    }
    i = i + 1;
}
return result;
        """
    },
    {
        "name": "complex break continue",
        "fn": test_complex_break_continue,
        "kaori": """
let result = 0;
let i = 0;
while i < 10 {
    let j = 0;
    while j < 10 {
        let k = 0;
        while k < 10 {
            if k == 3 {
                k = k + 1;
                continue;
            }
            if j == 7 and k == 8 {
                break;
            }
            if i == 5 and j == 5 and k == 5 {
                break;
            }
            result = result + 1;
            k = k + 1;
        }
        if i == 5 and j == 5 {
            break;
        }
        j = j + 1;
    }
    i = i + 1;
}
return result;
        """
    },
    {
        "name": "arithmetic expressions",
        "fn": test_arithmetic_expressions,
        "kaori": """
let x = 10;
let y = 7;
let z = 3;
let w = 5;
let v = 12;
let result = 0;
if x * 2 > y + w and v - z > x {
    if x * y > v * w or z + w > x {
        result = x * y * z + v * w;
    } else {
        result = x * v + y * z - w;
    }
} else {
    if v * 2 > x * y or z * w > x + y {
        result = x * w * z + y - v;
    } else {
        result = x * y + v * z - w;
    }
}
return result;
        """
    },
    {
        "name": "shadow variable in loop",
        "fn": test_shadow_variable_in_loop,
        "kaori": """
let result = 0;
let i = 8;
let j = 7;
while i < 10 {
    let j = 0;
    while j < 10 {
        if i == 5 and j == 5 {
            j = j + 1;
        } else {
            result = result + 1;
            j = j + 1;
        }
    }
    i = i + 1;
}
return result;
        """
    },
    {
        "name": "nested and conditions",
        "fn": test_nested_and_conditions,
        "kaori": """
let result = 0;
let i = 0;
while i < 20 {
    let j = 0;
    while j < 20 {
        if i > 5 and j > 5 and i + j > 15 {
            result = result + i * j;
        } else {
            result = result + 1;
        }
        j = j + 1;
    }
    i = i + 1;
}
return result;
        """
    },
    {
        "name": "nested or conditions",
        "fn": test_nested_or_conditions,
        "kaori": """
let result = 0;
let i = 0;
while i < 20 {
    let j = 0;
    while j < 20 {
        if i == 0 or j == 0 or i == 19 or j == 19 {
            result = result + 1;
        } else {
            result = result + i + j;
        }
        j = j + 1;
    }
    i = i + 1;
}
return result;
        """
    },
    {
        "name": "mixed and or",
        "fn": test_mixed_and_or,
        "kaori": """
let result = 0;
let i = 0;
while i < 15 {
    let j = 0;
    while j < 15 {
        if i > 3 and j > 3 or i > 10 or j > 10 {
            result = result + i * j;
        } else {
            result = result + i + j;
        }
        j = j + 1;
    }
    i = i + 1;
}
return result;
        """
    },
    {
        "name": "subtraction accumulator",
        "fn": test_subtraction_accumulator,
        "kaori": """
let result = 1000;
let i = 0;
while i < 50 {
    let j = 0;
    while j < 10 {
        if i > j {
            result = result - 1;
        } else {
            result = result + j - i;
        }
        j = j + 1;
    }
    i = i + 1;
}
return result;
        """
    },
    {
        "name": "multiply accumulator",
        "fn": test_multiply_accumulator,
        "kaori": """
let result = 0;
let i = 1;
while i < 8 {
    let j = 1;
    while j < 8 {
        let k = 1;
        while k < 8 {
            if i * j * k < 50 {
                result = result + i * j * k;
            }
            k = k + 1;
        }
        j = j + 1;
    }
    i = i + 1;
}
return result;
        """
    },
    {
        "name": "division condition",
        "fn": test_division_condition,
        "kaori": """
let result = 0;
let i = 1;
while i < 30 {
    let j = 1;
    while j < 30 {
        if i / j > 2 and i - j > 3 {
            result = result + i * j;
        } else {
            result = result + i + j;
        }
        j = j + 1;
    }
    i = i + 1;
}
return result;
        """
    },
    {
        "name": "less equal greater equal",
        "fn": test_less_equal_greater_equal,
        "kaori": """
let result = 0;
let i = 0;
while i < 20 {
    let j = 0;
    while j < 20 {
        if i >= 10 and j <= 10 {
            result = result + i - j;
        } else {
            result = result + j - i;
        }
        j = j + 1;
    }
    i = i + 1;
}
return result;
        """
    },
    {
        "name": "not equal condition",
        "fn": test_not_equal_condition,
        "kaori": """
let result = 0;
let i = 0;
while i < 20 {
    let j = 0;
    while j < 20 {
        if i != j and i != 0 and j != 0 {
            result = result + i + j;
        } else {
            result = result + 1;
        }
        j = j + 1;
    }
    i = i + 1;
}
return result;
        """
    },
    {
        "name": "complex arithmetic in condition",
        "fn": test_complex_arithmetic_in_condition,
        "kaori": """
let result = 0;
let i = 1;
while i < 15 {
    let j = 1;
    while j < 15 {
        let k = 1;
        while k < 15 {
            if i * j + k * 2 > i + j * k and i * k < j * j {
                result = result + i + j + k;
            } else {
                result = result + i * j - k;
            }
            k = k + 1;
        }
        j = j + 1;
    }
    i = i + 1;
}
return result;
        """
    },
    {
        "name": "deep nested if else with and or",
        "fn": test_deep_nested_if_else_with_and_or,
        "kaori": """
let x = 10;
let y = 7;
let z = 3;
let w = 5;
let v = 12;
let result = 0;
if x * 2 > y + w and v - z > x {
    if x * y > v * w or z + w > x {
        if x > 9 and w > 3 {
            result = x * y * w;
        } else {
            if z > 2 or y > 6 {
                result = x * y + z * w;
            } else {
                result = x + y + z + w;
            }
        }
    } else {
        if y > 6 and w > 4 {
            result = y * w + x;
        } else {
            result = x * z + y * w;
        }
    }
} else {
    if x > 5 or y > 6 {
        if y > 5 and z > 2 {
            result = y * z * w;
        } else {
            result = x * 2 + y;
        }
    } else {
        result = z * w * x;
    }
}
return result;
        """
    },
    {
        "name": "break on complex condition",
        "fn": test_break_on_complex_condition,
        "kaori": """
let result = 0;
let i = 0;
while i < 20 {
    let j = 0;
    while j < 20 {
        if i * j > 100 and i + j > 15 {
            break;
        }
        result = result + i + j;
        j = j + 1;
    }
    i = i + 1;
}
return result;
        """
    },
    {
        "name": "continue on complex condition",
        "fn": test_continue_on_complex_condition,
        "kaori": """
let result = 0;
let i = 0;
while i < 20 {
    let j = 0;
    while j < 20 {
        if i == j or i + j == 10 {
            j = j + 1;
            continue;
        }
        result = result + i * j;
        j = j + 1;
    }
    i = i + 1;
}
return result;
        """
    },
    {
        "name": "multiple breaks different depths",
        "fn": test_multiple_breaks_different_depths,
        "kaori": """
let result = 0;
let i = 0;
while i < 10 {
    let j = 0;
    while j < 10 {
        let k = 0;
        while k < 10 {
            if k > 5 {
                break;
            }
            result = result + k;
            k = k + 1;
        }
        if j > 5 {
            break;
        }
        result = result + j;
        j = j + 1;
    }
    if i > 5 {
        break;
    }
    result = result + i;
    i = i + 1;
}
return result;
        """
    },
    {
        "name": "alternating add subtract",
        "fn": test_alternating_add_subtract,
        "kaori": """
let result = 0;
let i = 0;
while i < 30 {
    let j = 0;
    while j < 30 {
        if i > j {
            result = result + i * 2 - j;
        } else {
            result = result + j * 2 - i;
        }
        j = j + 1;
    }
    i = i + 1;
}
return result;
        """
    },
    {
        "name": "triple and condition in loop",
        "fn": test_triple_and_condition_in_loop,
        "kaori": """
let result = 0;
let i = 0;
while i < 20 {
    let j = 0;
    while j < 20 {
        let k = 0;
        while k < 20 {
            if i > 5 and j > 5 and k > 5 {
                result = result + 1;
            } else {
                result = result + i + j + k;
            }
            k = k + 1;
        }
        j = j + 1;
    }
    i = i + 1;
}
return result;
        """
    },
    {
        "name": "harmonic sum condition",
        "fn": test_harmonic_sum_condition,
        "kaori": """
let result = 0;
let i = 1;
while i < 30 {
    let j = 1;
    while j < 30 {
        if i / j > 1 and i * j < 200 {
            result = result + i + j;
        } else {
            result = result + 1;
        }
        j = j + 1;
    }
    i = i + 1;
}
return result;
        """
    },
    {
        "name": "complex multi variable",
        "fn": test_complex_multi_variable,
        "kaori": """
let a = 3;
let b = 7;
let c = 11;
let d = 2;
let result = 0;
let i = 0;
while i < 25 {
    if i * a > b * d and i + c < 30 {
        result = result + i * a - b;
    } else {
        if i * b > c * d or i < a {
            result = result + i + b + c;
        } else {
            result = result + a * b - c * d;
        }
    }
    i = i + 1;
}
return result;
        """
    },
    {
        "name": "nested loop continue and break",
        "fn": test_nested_loop_continue_and_break,
        "kaori": """
let result = 0;
let i = 0;
while i < 15 {
    let j = 0;
    while j < 15 {
        if j == 3 or j == 7 {
            j = j + 1;
            continue;
        }
        if i == 10 and j > 10 {
            break;
        }
        result = result + i * j;
        j = j + 1;
    }
    i = i + 1;
}
return result;
        """
    },
    {
        "name": "accumulate with not equal and greater",
        "fn": test_accumulate_with_not_equal_and_greater,
        "kaori": """
let result = 0;
let i = 1;
while i < 25 {
    let j = 1;
    while j < 25 {
        if i != j and i > j and i * j > 50 {
            result = result + i - j;
        } else {
            if i != j and j > i and i * j > 50 {
                result = result + j - i;
            } else {
                result = result + 1;
            }
        }
        j = j + 1;
    }
    i = i + 1;
}
return result;
        """
    },
]