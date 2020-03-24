module.exports = {
    mode: "development",
    module: {
        rules: [
            {
                test: /\.(js|jsx)$/,
                exclude: /node_modules/,
                use: {
                    loader: "babel-loader",
                    options: {
                        presets: [
                            '@babel/react', {
                                'plugins': ['@babel/plugin-proposal-class-properties']
                            }
                        ]
                    }
                },
            },
            {
                test: /\.css$/i,
                use: ['style-loader', 'css-loader'],
            },
        ]
    },
    watchOptions: {
        poll: true,
        ignored: /node_modules/
    }
};