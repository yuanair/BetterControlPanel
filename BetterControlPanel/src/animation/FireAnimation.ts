// module


export type ColorScheme = [string, string, string];

export interface ParticleConfig {
    maxSize: number;
    minSpeed: number;
    maxSpeed: number;
    gravity: number;
    decayRate: number;
}

export class FlameParticle {
    private x: number;
    private y: number;
    private vx: number;
    private vy: number;
    private radius: number;
    private life: number;
    private decay: number;
    private config: ParticleConfig;

    constructor(
        private canvas: HTMLCanvasElement,
        config: ParticleConfig
    ) {
        this.x = Math.random() * this.canvas.width;
        this.y = this.canvas.height + Math.random() * 100;
        this.vx = (Math.random() - 0.5) * 2;
        this.vy = -Math.random() * config.maxSpeed - config.minSpeed;
        this.radius = Math.random() * config.maxSize;
        this.life = 1;
        this.decay = Math.random() * config.decayRate + 0.01;
        this.config = config;
    }

    update(): void {
        this.x += this.vx;
        this.y += this.vy;
        this.vy += this.config.gravity;
        this.life -= this.decay;
    }

    draw(ctx: CanvasRenderingContext2D, colors: ColorScheme): void {
        const gradient = ctx.createRadialGradient(
            this.x, this.y, 0,
            this.x, this.y, this.radius
        );

        gradient.addColorStop(0, colors[0]);
        gradient.addColorStop(0.5, colors[1]);
        gradient.addColorStop(1, colors[2]);

        ctx.beginPath();
        ctx.arc(this.x, this.y, this.radius, 0, Math.PI * 2);
        ctx.fillStyle = gradient;
        ctx.globalAlpha = this.life;
        ctx.fill();
    }
}

export class FlameAnimation {
    private particles: FlameParticle[] = [];
    private ctx: CanvasRenderingContext2D;
    private colorScheme: ColorScheme[] = [
        ['#FF3300', '#FF6600', '#FFCC00'],
        ['#00AAFF', '#5500FF', '#CC00FF']
    ];
    private colorMode: number = 0;
    private animationFrameId: number | null = null;

    constructor(
        private canvas: HTMLCanvasElement,
        private config: ParticleConfig = {
            maxSize: 10,
            minSpeed: 2,
            maxSpeed: 3,
            gravity: 0.1,
            decayRate: 0.015
        }
    ) {
        this.ctx = canvas.getContext('2d')!;
        this.init();
    }

    private init(): void {
        this.resizeCanvas();
        window.addEventListener('resize', () => this.resizeCanvas());
    }

    private resizeCanvas(): void {
        this.canvas.width = window.innerWidth;
        this.canvas.height = window.innerHeight;
    }

    createParticles(intensity: number): void {
        const particleCount = Math.floor(intensity * 2);

        if (this.particles.length < particleCount) {
            for (let i = this.particles.length; i < particleCount; i++) {
                this.particles.push(new FlameParticle(this.canvas, this.config));
            }
        } else {
            this.particles.length = particleCount;
        }
    }

    start(): void {
        if (!this.animationFrameId) {
            this.animate();
        }
    }

    stop(): void {
        if (this.animationFrameId) {
            cancelAnimationFrame(this.animationFrameId);
            this.animationFrameId = null;
        }
    }

    toggleColorMode(): void {
        this.colorMode = (this.colorMode + 1) % this.colorScheme.length;
    }

    private animate(): void {
        this.ctx.fillStyle = 'rgba(0, 0, 0, 0.1)';
        this.ctx.fillRect(0, 0, this.canvas.width, this.canvas.height);

        this.particles.forEach(particle => {
            particle.update();
            particle.draw(this.ctx, this.colorScheme[this.colorMode]);
        });

        this.animationFrameId = requestAnimationFrame(() => this.animate());
    }
}
