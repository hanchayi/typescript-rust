use criterion::{black_box, criterion_group, criterion_main, Criterion};
use ts_core::lexer::Lexer;

fn bench_lexer_simple(c: &mut Criterion) {
    let source = r#"
        function hello(name: string): string {
            return `Hello, ${name}!`;
        }
        
        const result = hello("TypeScript");
        console.log(result);
    "#;
    
    c.bench_function("lexer_simple", |b| {
        b.iter(|| {
            let mut lexer = Lexer::new(black_box(source));
            while let Some(_token) = lexer.next_token() {
                // Consume all tokens
            }
        })
    });
}

fn bench_lexer_complex(c: &mut Criterion) {
    let source = r#"
        interface User {
            id: number;
            name: string;
            email?: string;
        }
        
        class UserService {
            private users: User[] = [];
            
            async createUser(userData: Partial<User>): Promise<User> {
                const user: User = {
                    id: Math.random(),
                    name: userData.name || 'Unknown',
                    email: userData.email
                };
                this.users.push(user);
                return user;
            }
        }
    "#;
    
    c.bench_function("lexer_complex", |b| {
        b.iter(|| {
            let mut lexer = Lexer::new(black_box(source));
            while let Some(_token) = lexer.next_token() {
                // Consume all tokens
            }
        })
    });
}

criterion_group!(benches, bench_lexer_simple, bench_lexer_complex);
criterion_main!(benches);