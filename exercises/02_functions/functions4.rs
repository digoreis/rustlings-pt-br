// Esta loja está fazendo uma promoção onde, se o preço for um número par, você ganha 10
// Rustbucks de desconto, mas se for um número ímpar, são 3 Rustbucks de desconto.
// Não se preocupe com o corpo das funções, estamos interessados apenas nas
// assinaturas por enquanto.

fn is_even(num: i64) -> bool {
    num % 2 == 0
}

// TODO: Arrume a assinatura do método.
fn sale_price(price: i64) -> {
    if is_even(price) {
        price - 10
    } else {
        price - 3
    }
}

fn main() {
    let original_price = 51;
    println!("Seu preço de venda é {}", sale_price(original_price));
}
